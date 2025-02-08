mod games;

pub use games::perception::Perception;
pub use games::numeracy::Numeracy;
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::window;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    // Check which page we're on
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("no document on window");
    let location = window.location();
    let path = location.pathname().expect("pathname should exist");
    
    // Return the created instance rather than discarding it
    Ok(match path.as_str() {
        "/numeracy.html" => { Numeracy::new()?; },
        _ => { Perception::new()?; },
    })
}