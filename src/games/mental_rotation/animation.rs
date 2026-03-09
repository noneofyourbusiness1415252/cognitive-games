use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

/// Animate the rocket along `path` cells.
/// Uses CSS @keyframes injected into a <style> tag, so no JS is needed.
pub fn launch_rocket(document: &Document, path: &[(usize, usize)], _start: (usize, usize)) {
    if path.is_empty() { return; }

    // Get bounding rects via the grid and rocket elements
    let grid = match document.get_element_by_id("grid") {
        Some(g) => g,
        None => return,
    };
    let rocket_el = match document.query_selector(".rocket").ok().flatten() {
        Some(r) => r,
        None => return,
    };

    let grid_rect   = grid.get_bounding_client_rect();
    let rocket_rect = rocket_el.get_bounding_client_rect();

    let n_cols = path.iter().map(|c| c.0).max().unwrap_or(0) + 1;
    let cell_size = if n_cols > 0 { grid_rect.width() / n_cols as f64 } else { grid_rect.width().max(1.0) };

    let rocket_cx = rocket_rect.left() + rocket_rect.width() / 2.0;
    let rocket_cy = rocket_rect.top()  + rocket_rect.height() / 2.0;

    let mut css = String::from("@keyframes rocketTravel {\n");

    let total = path.len();
    for (i, &(px, py)) in path.iter().enumerate() {
        // Percentage: path cells occupy 0–85 %, Earth occupies 100 %
        let pct = if total > 1 { i as f64 / (total - 1) as f64 * 85.0 } else { 0.0 };
        let cell_cx = grid_rect.left() + (px as f64 + 0.5) * cell_size;
        let cell_cy = grid_rect.top()  + (py as f64 + 0.5) * cell_size;
        let dx = cell_cx - rocket_cx;
        let dy = cell_cy - rocket_cy;
        css.push_str(&format!("  {:.1}%{{transform:translate({:.1}px,{:.1}px)}}\n", pct, dx, dy));
    }

    // Land on Earth (right of end cell)
    let earth = document.query_selector(".earth").ok().flatten();
    let (earth_dx, earth_dy) = if let Some(e) = earth {
        let er = e.get_bounding_client_rect();
        (er.left() + er.width() / 2.0 - rocket_cx, er.top() + er.height() / 2.0 - rocket_cy)
    } else {
        let &(ex, ey) = path.last().unwrap();
        let cx = grid_rect.left() + (ex as f64 + 1.5) * cell_size;
        let cy = grid_rect.top()  + (ey as f64 + 0.5) * cell_size;
        (cx - rocket_cx, cy - rocket_cy)
    };
    css.push_str(&format!("  100%{{transform:translate({:.1}px,{:.1}px)}}\n", earth_dx, earth_dy));
    css.push_str("}\n");

    // Inject keyframes into a <style> element
    if let Ok(style_el) = document.create_element("style") {
        style_el.set_text_content(Some(&css));
        if let Some(head) = document.query_selector("head").ok().flatten() {
            let _ = head.append_child(&style_el);
        }
    }

    // Apply animation + lock pointer events
    if let Ok(rocket_html) = rocket_el.clone().dyn_into::<HtmlElement>() {
        let style = rocket_html.style();
        let _ = style.set_property("animation", "rocketTravel 3s linear forwards");
    }
    if let Some(container) = document.query_selector(".grid-container").ok().flatten() {
        let _ = container.class_list().add_1("animating");
    }
}
