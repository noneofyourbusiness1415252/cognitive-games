mod games;

pub use games::numeracy::Numeracy;
pub use games::perception::Perception;
use wasm_bindgen::{prelude::*, JsValue};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Check which page we're on
    let window = web_sys::window().expect("no global window exists");
    let location = window.location();
    let path = location.pathname().expect("pathname should exist");

    // Return the created instance rather than discarding it
    match path.as_str() {
        "/numeracy" => {
            let game = Numeracy::new()?;
            game.start()?;
        }
        _ => {
            Perception::new()?;
        }
    }
    Ok(())
}
