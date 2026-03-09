mod tile;
mod level_generator;
mod grid;
mod timer;
mod rotation;
mod animation;

use tile::Direction;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Event, MouseEvent, Window};
use wasm_bindgen::JsCast;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Option<MentalRotation>> = Mutex::new(None);
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct MentalRotation {
    level:          usize,
    tiles:          Vec<tile::Tile>,
    initial_tiles:  Vec<tile::Tile>,
    grid_size:      usize,
    start_pos:      (usize, usize),
    end_pos:        (usize, usize),
    moves:          usize,
    rotations:      usize,
    reversals:      usize,
    time_remaining: u32,
    #[serde(skip)]
    last_click:     f64,
}

// ── WASM-exported methods ────────────────────────────────────────────────────
#[wasm_bindgen]
impl MentalRotation {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new(level: usize) -> Self {
        if level == 1 {
            if let Some(saved) = load_state() { return saved; }
        }
        let n = level.max(1);
        let (tiles, _path, start, end) = level_generator::generate_level(n);
        let initial = tiles.clone();
        Self {
            level, tiles, initial_tiles: initial, grid_size: n,
            start_pos: start, end_pos: end,
            moves: 0, rotations: 0, reversals: 0,
            time_remaining: 180, last_click: 0.0,
        }
    }

    pub fn start(&self) -> Result<(), JsValue> {
        if let Ok(mut lock) = GAME.try_lock() {
            if let Some(old) = lock.take() { old.clear_state(); }
        }
        let window = web_sys::window().unwrap();
        let doc = window.document().unwrap();
        update_level_display(&doc, self.level);
        self.render_grid(&doc)?;
        self.setup_timer(&window)?;
        self.setup_reset(&doc)?;
        if let Ok(mut lock) = GAME.try_lock() {
            *lock = Some(self.clone());
            self.save_state();
        }
        Ok(())
    }

    pub fn save_state(&self) {
        if let Some(w) = web_sys::window() {
            if let Some(s) = w.local_storage().ok().flatten() {
                if let Ok(j) = serde_json::to_string(self) {
                    let _ = s.set_item("mental_rotation_state", &j);
                }
            }
        }
    }

    pub fn clear_state(&self) {
        if let Some(w) = web_sys::window() {
            if let Some(s) = w.local_storage().ok().flatten() {
                let _ = s.remove_item("mental_rotation_state");
            }
            if let Some(h) = unsafe { timer::TIMER_HANDLE } {
                w.clear_interval_with_handle(h);
                unsafe { timer::TIMER_HANDLE = None; }
            }
        }
    }

    #[wasm_bindgen(getter)]
    #[must_use] pub fn grid_size(&self) -> usize { self.grid_size }
}

// ── Internal methods ─────────────────────────────────────────────────────────
impl MentalRotation {

    fn render_grid(&self, doc: &Document) -> Result<(), JsValue> {
        let grid = doc.get_element_by_id("grid").unwrap();
        while let Some(c) = grid.first_child() { grid.remove_child(&c)?; }

        grid.set_attribute("style", &format!(
            "grid-template-columns: repeat({}, calc(1em * sqrt(2)))", self.grid_size))?;

        {
            let cb = Closure::wrap(Box::new(|e: Event| {
                e.prevent_default(); e.stop_propagation();
            }) as Box<dyn FnMut(Event)>);
            grid.add_event_listener_with_callback("contextmenu", cb.as_ref().unchecked_ref())?;
            cb.forget();
        }

        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                let cell = doc.create_element("div")?;
                cell.set_attribute("data-col", &x.to_string())?;
                cell.set_attribute("data-row", &y.to_string())?;
                cell.set_class_name("cell");

                if let Some((ti, ci)) = self.tile_at(x, y) {
                    let t = &self.tiles[ti];
                    if t.is_obstacle {
                        cell.set_class_name("cell obstacle");
                    } else {
                        cell.set_class_name("cell tile");
                        cell.set_attribute("data-tile", &ti.to_string())?;
                        let arrow = doc.create_element("span")?;
                        arrow.set_class_name(t.arrows[ci].css_class());
                        arrow.set_text_content(Some("➔"));
                        cell.append_child(&arrow)?;
                    }
                }
                grid.append_child(&cell)?;
            }
        }

        for sel in &[".rocket", ".earth"] {
            if let Some(e) = doc.query_selector(sel)?.as_ref() { e.remove(); }
        }
        let container = doc.query_selector(".grid-container")?.unwrap();
        let rocket = doc.create_element("span")?;
        let earth  = doc.create_element("span")?;
        rocket.set_class_name("rocket"); rocket.set_text_content(Some("🚀"));
        earth.set_class_name("earth");   earth.set_text_content(Some("🌍"));
        if self.grid_size > 1 {
            let sy = self.start_pos.1 as f64 / (self.grid_size - 1) as f64 * 100.0;
            let ey = self.end_pos.1   as f64 / (self.grid_size - 1) as f64 * 100.0;
            rocket.set_attribute("style", &format!("top:{sy:.1}%"))?;
            earth.set_attribute("style",  &format!("top:{ey:.1}%"))?;
        }
        container.append_child(&rocket)?;
        container.append_child(&earth)?;

        let click_cb = Closure::wrap(Box::new(move |e: MouseEvent| {
            e.prevent_default(); e.stop_propagation();
            let target = match e.target().and_then(|t| t.dyn_into::<Element>().ok()) {
                Some(t) => t, None => return,
            };
            let tile_el = if target.class_list().contains("tile") { target.clone() }
                else if let Some(p) = target.parent_element() {
                    if p.class_list().contains("tile") { p } else { return; }
                } else { return; };

            let idx: usize = match tile_el.get_attribute("data-tile")
                .and_then(|s| s.parse().ok()) { Some(i) => i, None => return };

            if let Ok(mut lock) = GAME.try_lock() {
                if let Some(mut game) = lock.take() {
                    let now = js_sys::Date::now();
                    if now - game.last_click < 100.0 { *lock = Some(game); return; }
                    game.last_click = now;

                    if e.button() == 0 { game.do_rotate(idx); }
                    else if e.button() == 2 { game.do_reverse(idx); }

                    if let Some(window) = web_sys::window() {
                        if let Some(doc) = window.document() {
                            game.refresh_arrows(&doc, idx);
                            update_stats(&doc, game.moves, game.rotations, game.reversals);
                            if let Some(path) = game.winning_path() {
                                animation::launch_rocket(&doc, &path, game.start_pos);
                                let nl = game.level + 1;
                                game.save_state();
                                *lock = Some(game);
                                schedule_next_level(nl);
                                return;
                            }
                        }
                    }
                    game.save_state();
                    *lock = Some(game);
                }
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        grid.add_event_listener_with_callback("mousedown", click_cb.as_ref().unchecked_ref())?;
        click_cb.forget();
        Ok(())
    }

    fn refresh_arrows(&self, doc: &Document, ti: usize) {
        if let Some(tile) = self.tiles.get(ti) {
            for (ci, &(x, y)) in tile.cells.iter().enumerate() {
                let sel = format!(".cell[data-col='{x}'][data-row='{y}'] .arrow");
                if let Some(arrow) = doc.query_selector(&sel).ok().flatten() {
                    if let Some(&a) = tile.arrows.get(ci) {
                        arrow.set_class_name(a.css_class());
                    }
                }
            }
        }
    }

    fn tile_at(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        for (ti, t) in self.tiles.iter().enumerate() {
            for (ci, &c) in t.cells.iter().enumerate() {
                if c == (x, y) { return Some((ti, ci)); }
            }
        }
        None
    }

    fn do_rotate(&mut self, ti: usize) {
        let rotated = match self.tiles.get(ti) { Some(t) => t.rotated_cells(), None => return };
        if !self.valid_placement(&rotated, ti) { return; }
        self.tiles[ti].rotate_cw();
        self.moves += 1; self.rotations += 1;
    }

    fn do_reverse(&mut self, ti: usize) {
        if let Some(t) = self.tiles.get_mut(ti) {
            t.reverse_arrows(); self.moves += 1; self.reversals += 1;
        }
    }

    fn valid_placement(&self, cells: &[(usize, usize)], exclude: usize) -> bool {
        cells.iter().all(|&(x, y)| x < self.grid_size && y < self.grid_size)
        && self.tiles.iter().enumerate().all(|(i, t)| {
            i == exclude || cells.iter().all(|c| !t.cells.contains(c))
        })
    }

    /// Follow arrows from start_pos; return cell sequence if path reaches end_pos (East exit).
    fn winning_path(&self) -> Option<Vec<(usize, usize)>> {
        let mut pos = self.start_pos;
        let mut travel = Direction::East;
        let mut path = vec![pos];
        let limit = self.grid_size * self.grid_size + 2;

        for _ in 0..limit {
            let (ti, ci) = self.tile_at(pos.0, pos.1)?;
            let tile = &self.tiles[ti];
            if tile.is_obstacle { return None; }
            let arrow = *tile.arrows.get(ci)?;

            let exit = match arrow.delta() {
                Some(_) => arrow,
                None => {
                    let (c1, c2) = arrow.components()?;
                    if c1 == travel { c2 } else if c2 == travel { c1 } else { return None; }
                }
            };

            if pos == self.end_pos {
                if exit == Direction::East { return Some(path); }
                return None;
            }

            let (dx, dy) = exit.delta()?;
            let nx = pos.0 as i32 + dx;
            let ny = pos.1 as i32 + dy;
            if nx < 0 || ny < 0 { return None; }
            let next = (nx as usize, ny as usize);
            if next.0 >= self.grid_size || next.1 >= self.grid_size { return None; }
            if path.contains(&next) { return None; }

            path.push(next);
            travel = exit;
            pos = next;
        }
        None
    }

    fn setup_timer(&self, window: &Window) -> Result<(), JsValue> {
        timer::setup_timer(window, self.time_remaining)
    }

    fn setup_reset(&self, doc: &Document) -> Result<(), JsValue> {
        if let Some(btn) = doc.get_element_by_id("reset") {
            let cb = Closure::wrap(Box::new(move |_: Event| {
                if let Ok(mut lock) = GAME.try_lock() {
                    if let Some(mut g) = lock.take() {
                        g.tiles = g.initial_tiles.clone();
                        g.moves = 0; g.rotations = 0; g.reversals = 0;
                        *lock = Some(g.clone());
                        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                            let _ = g.render_grid(&doc);
                            update_stats(&doc, 0, 0, 0);
                        }
                        g.save_state();
                    }
                }
            }) as Box<dyn FnMut(Event)>);
            btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
            cb.forget();
        }
        Ok(())
    }
}

// ── Module helpers ───────────────────────────────────────────────────────────

fn update_level_display(doc: &Document, level: usize) {
    if let Some(el) = doc.query_selector(".level").ok().flatten() {
        el.set_text_content(Some(&format!("Level {level}")));
    }
}

fn update_stats(doc: &Document, moves: usize, rots: usize, revs: usize) {
    if let Some(el) = doc.get_element_by_id("stats") {
        el.set_text_content(Some(
            &format!("Moves: {moves}  |  Rotations: {rots}  |  Reversals: {revs}")
        ));
    }
}

fn schedule_next_level(next: usize) {
    let cb = Closure::once_into_js(move || {
        let g = MentalRotation::new(next);
        let _ = g.start();
    });
    if let Some(w) = web_sys::window() {
        let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(), 3200);
    }
}

fn load_state() -> Option<MentalRotation> {
    let storage = web_sys::window()?.local_storage().ok()??;
    let json = storage.get_item("mental_rotation_state").ok()??;
    serde_json::from_str(&json).ok()
}
