use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Window};
use crate::games::mental_rotation::GAME_INSTANCE;

// Make the timer handle public so it can be accessed from the main module
pub static mut TIMER_HANDLE: Option<i32> = None;

pub fn setup_timer(window: &Window, initial_seconds: u32) -> Result<(), JsValue> {
    // Clear any existing timer
    if let Some(handle) = unsafe { TIMER_HANDLE } {
        window.clear_interval_with_handle(handle);
        unsafe { TIMER_HANDLE = None; }
    }
    
    let document = window.document().unwrap();
    let timer_element = document.query_selector(".timer")?.unwrap();
    
    // Format and set the initial time
    let mins = initial_seconds / 60;
    let secs = initial_seconds % 60;
    timer_element.set_text_content(Some(&format!("{:01}:{:02}", mins, secs)));
    
    // Create closure for the timer update
    let timer_callback = Closure::wrap(Box::new(move || {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(timer_element) = document.query_selector(".timer").ok().flatten() {
                    if let Ok(mut game_lock) = GAME_INSTANCE.try_lock() {
                        if let Some(mut game) = game_lock.take() {
                            // Decrease time by 1 second if greater than 0
                            if game.time_remaining > 0 {
                                game.time_remaining -= 1;
                                
                                // Update timer display
                                let mins = game.time_remaining / 60;
                                let secs = game.time_remaining % 60;
                                timer_element.set_text_content(Some(&format!("{:01}:{:02}", mins, secs)));
                                
                                // Save state after updating the time
                                game.save_state();
                                
                                *game_lock = Some(game);
                            } else {
                                // Time's up - reset the timer first to prevent recursive calls
                                if let Some(handle) = unsafe { TIMER_HANDLE } {
                                    window.clear_interval_with_handle(handle);
                                    unsafe { TIMER_HANDLE = None; }
                                }
                                
                                let current_level = game.level;
                                game.clear_game_state();
                                *game_lock = None; // Clear the game instance before creating new one
                                
                                // Create and start new game with same level outside of the lock
                                // Fix: Use closure.as_ref().unchecked_ref() to get proper type
                                let timeout_callback = Closure::once(move || {
                                    let new_game = crate::games::mental_rotation::MentalRotation::new(current_level);
                                    let _ = new_game.start();
                                });
                                
                                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                                    timeout_callback.as_ref().unchecked_ref(), 
                                    100 // Small delay to ensure clean transition
                                );
                                
                                // Prevent the closure from being dropped
                                timeout_callback.forget();
                            }
                        }
                    }
                }
            }
        }
    }) as Box<dyn FnMut()>);
    
    // Set 1-second interval
    let handle = window.set_interval_with_callback_and_timeout_and_arguments_0(
        timer_callback.as_ref().unchecked_ref(), 
        1000
    )?;
    
    // Store the interval handle for later use
    set_timer_handle(handle);
    
    // Keep the closure alive
    timer_callback.forget();
    
    Ok(())
}

pub fn set_timer_handle(handle: i32) {
    unsafe {
        TIMER_HANDLE = Some(handle);
    }
}

pub fn get_timer_handle() -> i32 {
    unsafe {
        TIMER_HANDLE.unwrap_or(0)
    }
}
