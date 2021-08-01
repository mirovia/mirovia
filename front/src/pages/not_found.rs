use crate::parts;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
pub fn go() -> Result<(), JsValue> {
    parts::top::add()?;
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    body.set_id("not_found_page_body");
    let title = document
        .create_element("h1")?
        .dyn_into::<web_sys::HtmlHeadingElement>()?;
    title.set_text_content(Some("404 - Page not found"));
    body.append_child(&title)?;
    Ok(())
}
