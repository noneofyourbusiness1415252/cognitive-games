use wasm_bindgen::prelude::*;
use web_sys::Window;
use std::rc::Rc;
use std::cell::Cell;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref CURRENT_TIMER: Mutex<Option<i32>> = Mutex::new(None);
}

pub fn setup_timer(window: &Window, initial_time: u32) -> Result<(), JsValue> {
    // Clear any existing timer
    if let Some(id) = CURRENT_TIMER.lock().unwrap().take() {
        window.clear_interval_with_handle(id);
    }
    
    let timer_element = window.document()
        .unwrap()
        .query_selector(".timer")
        .unwrap()
        .unwrap();

    let time = Rc::new(Cell::new(initial_time));
    let timer_element = Rc::new(timer_element);
    
    let timer_element_clone = timer_element.clone();
    let time_clone = time.clone();
    
    let callback = Closure::wrap(Box::new(move || {
        let current = time_clone.get();
        if current > 0 {
            time_clone.set(current - 1);
            let mins = current / 60;
            let secs = current % 60;
            timer_element_clone.set_text_content(Some(&format!("{}:{:02}", mins, secs)));
        }
    }) as Box<dyn FnMut()>);

    let handle = window.set_interval_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        1000,
    )?;
    
    // Store new timer ID
    *CURRENT_TIMER.lock().unwrap() = Some(handle);
    
    callback.forget();
    
    Ok(())
}
