use crate::error;
use crate::hashmap;
use crate::log;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use web_sys::HtmlDivElement;
use web_sys::HtmlTextAreaElement;
use web_sys::Storage;
pub fn build() -> Result<web_sys::HtmlDivElement, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let debug_div = document
        .create_element("div")?
        .dyn_into::<HtmlDivElement>()?;
    debug_div.set_id("debug");
    //
    let debug_div_top = document
        .create_element("div")?
        .dyn_into::<HtmlDivElement>()?;
    debug_div_top.set_id("debug_div_top");
    debug_div.append_child(&debug_div_top);
    // Show local storage
    let local_storage_button = document
        .create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;
    local_storage_button.set_inner_text("Show local storage");
    let local_storage_button_onclick =
        Closure::wrap(Box::new(move || print_local_storage()) as Box<dyn FnMut()>);
    local_storage_button.set_id("print_local_storage_button");
    local_storage_button.set_onclick(Some(local_storage_button_onclick.as_ref().unchecked_ref()));
    // TODO: do not use forget
    local_storage_button_onclick.forget();
    debug_div_top.append_child(&local_storage_button);
    //
    let test_debug_button = document
        .create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;
    debug_div_top.append_child(&test_debug_button);
    test_debug_button.set_inner_text("Test");
    //
    let debug_div_logs = document
        .create_element("div")?
        .dyn_into::<HtmlDivElement>()?;
    debug_div_logs.set_id("debug_div_logs");
    debug_div.append_child(&debug_div_logs);
    // textarea
    let debug_textarea = document
        .create_element("textarea")?
        .dyn_into::<HtmlTextAreaElement>()?;
    debug_textarea.set_id("debug_textarea");
    debug_div.append_child(&debug_textarea);
    load_debug_visibility(&debug_div);
    Ok(debug_div)
}
fn print_local_storage() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let debug_div_logs:HtmlDivElement = document
        .get_element_by_id("debug_div_logs")
        .unwrap()
        .dyn_into::<HtmlDivElement>()
        .unwrap();
    let storage: Storage = window.local_storage().unwrap().unwrap();
    debug_div_logs.set_inner_text(&(debug_div_logs.inner_text() + "local storage:\n"));
    for i in 0..storage.length().unwrap() {
        let key = &storage.key(i).unwrap().unwrap();
        let value = storage.get_item(key).unwrap().unwrap();
        debug_div_logs.set_inner_text(&(debug_div_logs.inner_text() + &format!("  {}: {}",key,value) + "\n"));
    }

    debug_div_logs.set_scroll_top(debug_div_logs.scroll_height());
    //log("local_storage");
}
pub fn load_debug_visibility(debug_div: &web_sys::HtmlDivElement) {
    let window = web_sys::window().expect("no global `window` exists");
    let storage: Storage = window.local_storage().unwrap().unwrap();
    match storage.get_item("debug_visibility").unwrap() {
        Some(visibility) => set_debug_visibility(debug_div, &visibility),
        None => set_debug_visibility(debug_div, "visible_false"),
    };
}
pub fn current_debug_visibility() -> &'static str {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let debug: web_sys::HtmlDivElement = document
        .get_element_by_id("debug")
        .unwrap()
        .dyn_into::<web_sys::HtmlDivElement>()
        .unwrap();
    if debug.class_list().contains("visible_true") {
        return "visible_true";
    }
    if debug.class_list().contains("visible_false") {
        return "visible_false";
    }
    return "error";
}
fn set_debug_visibility(debug_div: &web_sys::HtmlDivElement, visibility: &str) {
    debug_div.class_list().add_1(visibility).unwrap();
    save_debug_visibility(visibility);
}
fn save_debug_visibility(visibility: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let storage: Storage = window.local_storage().unwrap().unwrap();
    storage.set_item("debug_visibility", &visibility).unwrap();
}
pub fn cycle_debug_visibility() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    match document.get_element_by_id("debug") {
        Some(debug_a) => {
            let debug = debug_a.dyn_into::<web_sys::HtmlDivElement>().unwrap();
            let debug_visibility_cycling: HashMap<String, String> = hashmap!["visible_false".to_owned()=>"visible_true".to_owned(), "visible_true".to_owned()=>"visible_false".to_owned()];
            let current_visibility = current_debug_visibility();
            let new_visibility = &debug_visibility_cycling[&current_visibility.to_string()];
            match debug
                .class_list()
                .replace(&current_visibility, &new_visibility)
                .unwrap()
            {
                false => {
                    log("error");
                }
                true => {
                    save_debug_visibility(new_visibility);
                }
            }
        }
        None => error("no debug on this page"),
    }
}
