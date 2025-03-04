use super::Perception;
use js_sys::Date;
use wasm_bindgen::prelude::*;

impl Perception {
    pub(super) fn save_state(&self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global window exists");
        let storage = window.local_storage()?.expect("no local storage exists");

        let state = serde_wasm_bindgen::to_value(&self)?;
        let state_json = js_sys::JSON::stringify(&state)?.as_string().unwrap();
        storage.set_item("maze_state", &state_json)?;
        storage.set_item("maze_time", &Date::now().to_string())?;
        storage.set_item("maze_level", &self.level.to_string())?;

        Ok(())
    }
}
