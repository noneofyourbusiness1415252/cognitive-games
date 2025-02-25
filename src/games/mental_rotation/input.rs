use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref LAST_CLICK_TIME: Mutex<f64> = Mutex::new(0.0);
}

pub fn should_handle_click(current_time: f64) -> bool {
    if let Ok(mut last_time) = LAST_CLICK_TIME.try_lock() {
        if current_time - *last_time < 100.0 {
            return false;
        }
        *last_time = current_time;
        true
    } else {
        false
    }
}
