use crate::parts;
use wasm_bindgen::JsValue;
pub fn go() -> Result<(), JsValue> {
    parts::top::add()?;
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    body.set_id("forgot_page_body");
    Ok(())
}
