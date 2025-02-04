use web_sys::Document;

pub fn get_document() -> Document {
    web_sys::window()
        .expect("no global window exists")
        .document()
        .expect("no document exists")
}