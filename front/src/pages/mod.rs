use crate::parts;
pub mod enter;
pub mod forgot_credentials;
pub mod home;
pub mod not_found;
pub mod playground;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
//use crate::parts::debug::load_debug_visibility;
pub fn build(content: Result<web_sys::HtmlDivElement, JsValue>) -> Result<(), JsValue> {
    parts::top::add()?;
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let debug_div = parts::debug::build().unwrap();
    body.append_child(&debug_div)?;
    let middle_div = document
        .create_element("div")?
        .dyn_into::<web_sys::HtmlDivElement>()?;
    middle_div.set_id("middle");
    let content_div = content.unwrap();
    middle_div.append_child(&content_div)?;
    middle_div.append_child(&debug_div)?;
    body.append_child(&middle_div)?;
    Ok(())
}
