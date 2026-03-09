use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Window;
use crate::games::mental_rotation::GAME;

pub static mut TIMER_HANDLE: Option<i32> = None;

pub fn setup_timer(window: &Window, initial_secs: u32) -> Result<(), JsValue> {
    // Clear any existing timer
    if let Some(h) = unsafe { TIMER_HANDLE } {
        window.clear_interval_with_handle(h);
        unsafe { TIMER_HANDLE = None; }
    }
    let doc = window.document().unwrap();
    if let Some(el) = doc.query_selector(".timer").ok().flatten() {
        let m = initial_secs / 60;
        let s = initial_secs % 60;
        el.set_text_content(Some(&format!("{m}:{s:02}")));
    }

    let cb = Closure::wrap(Box::new(move || {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(timer_el) = doc.query_selector(".timer").ok().flatten() {
                    if let Ok(mut lock) = GAME.try_lock() {
                        if let Some(mut g) = lock.take() {
                            if g.time_remaining > 0 {
                                g.time_remaining -= 1;
                                let m = g.time_remaining / 60;
                                let s = g.time_remaining % 60;
                                timer_el.set_text_content(Some(&format!("{m}:{s:02}")));
                                g.save_state();
                                *lock = Some(g);
                            } else {
                                if let Some(h) = unsafe { TIMER_HANDLE } {
                                    window.clear_interval_with_handle(h);
                                    unsafe { TIMER_HANDLE = None; }
                                }
                                let lv = g.level;
                                g.clear_state();
                                *lock = None;
                                let cb2 = Closure::once(move || {
                                    let ng = crate::games::mental_rotation::MentalRotation::new(lv);
                                    let _ = ng.start();
                                });
                                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                                    cb2.as_ref().unchecked_ref(), 100);
                                cb2.forget();
                            }
                        }
                    }
                }
            }
        }
    }) as Box<dyn FnMut()>);

    let h = window.set_interval_with_callback_and_timeout_and_arguments_0(
        cb.as_ref().unchecked_ref(), 1000)?;
    unsafe { TIMER_HANDLE = Some(h); }
    cb.forget();
    Ok(())
}
