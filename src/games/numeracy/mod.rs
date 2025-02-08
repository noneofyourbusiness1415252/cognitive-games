use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement};

mod expression;
mod level;
mod state;

use expression::Expression;
use level::Level;
use state::GameState;

#[wasm_bindgen]
pub struct Numeracy {
    state: GameState,
    document: Document,
    container: HtmlElement,
}

#[wasm_bindgen]
impl Numeracy {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Numeracy, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let container = document
            .get_element_by_id("game-container")
            .unwrap()
            .dyn_into::<HtmlElement>()?;

        Ok(Numeracy {
            state: GameState::new(),
            document,
            container,
        })
    }
}