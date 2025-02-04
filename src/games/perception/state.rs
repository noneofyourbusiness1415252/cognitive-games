use super::Perception;
use js_sys::Date;
use wasm_bindgen::prelude::*;
use web_sys::Storage;

impl Perception {
    pub(super) fn load_state(storage: &Storage) -> Result<Option<(Self, f64)>, JsValue> {
        if let Some(state) = storage.get_item("maze_state")? {
            let last_save = storage
                .get_item("maze_time")?
                .unwrap_or_else(|| "0".to_string())
                .parse::<f64>()
                .unwrap_or(0.0);
            
            let game: Self = serde_wasm_bindgen::from_value(js_sys::JSON::parse(&state)?)?;
            Ok(Some((game, last_save)))
        } else {
            Ok(None)
        }
    }

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