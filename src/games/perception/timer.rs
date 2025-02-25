use super::Perception;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::console;

impl Perception {
    pub(super) fn setup_timer(game_state: &Rc<RefCell<Perception>>) -> Result<(), JsValue> {
        let timer_callback = {
            let game_state = game_state.clone();
            Closure::wrap(Box::new(move || {
                if let Ok(mut game) = game_state.try_borrow_mut() {
                    let now = js_sys::Date::now() / 1000.0;
                    let delta = (now - game.last_tick) as i32;
                    if delta >= 1 {
                        game.update_timer(now);
                    }
                }
            }) as Box<dyn FnMut()>)
        };

        let window = web_sys::window().unwrap();
        console::log_1(&"Setting up interval...".into());

        let result = window.set_interval_with_callback_and_timeout_and_arguments_0(
            timer_callback.as_ref().unchecked_ref(),
            1000,
        );

        match result {
            Ok(_) => console::log_1(&"Interval set up successfully".into()),
            Err(e) => console::log_2(&"Failed to set up interval:".into(), &e),
        }

        timer_callback.forget();
        Ok(())
    }

    fn update_timer(&mut self, now: f64) {
        self.time_remaining -= 1;
        self.last_tick = now;

        if self.time_remaining <= 0 {
            self.reset_on_timeout(now);
        }

        self.update_timer_display();
        self.save_state().unwrap_or_else(|_| {
            console::log_1(&"Failed to save game state".into());
        });
    }

    fn update_timer_display(&self) {
        if let Some(timer_el) = self.document.get_element_by_id("timer") {
            let minutes = self.time_remaining / 60;
            let seconds = self.time_remaining % 60;
            timer_el.set_text_content(Some(&format!("{minutes}:{seconds:02}")));
        }
    }

    fn reset_on_timeout(&mut self, now: f64) {
        let new_game = Self::create_maze(self.size, self.document.clone());
        self.walls = new_game.walls;
        self.key_position = new_game.key_position;
        self.door_position = new_game.door_position;
        self.reset_position();
        self.time_remaining = 300;
        self.last_tick = now;
        self.render().unwrap();
    }
}
