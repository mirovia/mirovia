use crate::parts;
use wasm_bindgen::JsValue;
// TODO: add a `selected` parameter, to highlight a link in the nav bar so that
// user know where they are
pub fn add() -> Result<(), JsValue> {
    parts::top_navigation::add()?;
    parts::top_tools::add()?;
    Ok(())
}
