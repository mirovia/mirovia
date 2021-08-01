use crate::parts;
use wasm_bindgen::JsValue;

pub fn add() -> Result<(), JsValue> {
    parts::top_navigation::add()?;
    parts::tools::add()?;
    Ok(())
}
