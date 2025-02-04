mod games;
mod utils;

pub use games::perception::Perception;
use js_sys::Math;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let game = Perception::new()?;
    game.render()?;
    Ok(())
}