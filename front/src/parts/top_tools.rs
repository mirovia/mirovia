use crate::error;
use crate::hashmap;
// use crate::log;
use std::collections::HashMap;
use url::Url;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlButtonElement;
use web_sys::HtmlImageElement;
// use web_sys::HtmlDivElement;
use crate::parts;
use web_sys::HtmlLinkElement;
use web_sys::HtmlSpanElement;
use web_sys::Storage;
pub fn add() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let tool_div = document.create_element("div")?;
    tool_div.set_id("tool_div");
    body.append_child(&tool_div)?;
    let spacer: HtmlSpanElement = document
        .create_element("span")?
        .dyn_into::<web_sys::HtmlSpanElement>()?;
    spacer.set_class_name("flex-spacer");
    tool_div.append_child(&spacer)?;
    //
    tool_div.append_child(&switch_theme_button().unwrap())?;
    tool_div.append_child(&debug_button().unwrap())?;
    tool_div.append_child(&switch_language_button().unwrap())?;
    Ok(())
}
pub fn debug_button() -> Result<HtmlButtonElement, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let button_onclick =
        Closure::wrap(Box::new(move || parts::debug::cycle_debug_visibility()) as Box<dyn FnMut()>);
    let button = document
        .create_element("button")?
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    button.set_onclick(Some(button_onclick.as_ref().unchecked_ref()));
    // TODO: do not use forget
    button_onclick.forget();
    button.set_inner_text("Debug");
    return Ok(button);
}
pub fn switch_theme_button() -> Result<HtmlButtonElement, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let switch_theme_button_onclick =
        Closure::wrap(Box::new(move || switch_theme()) as Box<dyn FnMut()>);
    let switch_theme_button = document
        .create_element("button")?
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    switch_theme_button.set_onclick(Some(switch_theme_button_onclick.as_ref().unchecked_ref()));
    // TODO: do something with resulting JsValue
    switch_theme_button_onclick.into_js_value();
    switch_theme_button.set_inner_text("Theme");
    return Ok(switch_theme_button);
}
pub fn switch_theme() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let theme: HtmlLinkElement = document
        .get_element_by_id("theme")
        .unwrap()
        .dyn_into::<web_sys::HtmlLinkElement>()
        .unwrap();
    let theme_cycling: HashMap<String, String> = hashmap!["/index_light.css".to_owned()=>"/index_dark.css".to_owned(), "/index_dark.css".to_owned()=>"/index_light.css".to_owned()];
    let new_theme = &theme_cycling[Url::parse(&theme.href()).unwrap().path()];
    set_theme(new_theme);
}
pub fn set_theme(theme_value: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    match document.get_element_by_id("theme") {
        Some(theme_element) => {
            let theme: HtmlLinkElement = theme_element.dyn_into::<HtmlLinkElement>().unwrap();
            theme.set_href(&theme_value);
        }
        None => {
            error("<link id=\"theme\"> not found");
        }
    };
    let storage: Storage = window.local_storage().unwrap().unwrap();
    storage.set_item("theme", &theme_value).unwrap();
}
pub fn load_theme() {
    let window = web_sys::window().expect("no global `window` exists");
    let storage: Storage = window.local_storage().unwrap().unwrap();
    match storage.get_item("theme").unwrap() {
        Some(theme) => set_theme(&theme),
        None => set_theme("/index_light.css"),
    };
}

pub fn switch_language_button() -> Result<HtmlButtonElement, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let switch_language_button_onclick =
        Closure::wrap(Box::new(move || switch_language()) as Box<dyn FnMut()>);
    let switch_language_button = document
        .create_element("button")?
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    switch_language_button.set_id("language_button");
    switch_language_button.set_onclick(Some(
        switch_language_button_onclick.as_ref().unchecked_ref(),
    ));
    // TODO: do something with resulting JsValue
    switch_language_button_onclick.into_js_value();
    //switch_language_button.set_inner_text("Language");
    let img = document
        .create_element("img")?
        .dyn_into::<HtmlImageElement>()?;
    img.set_src("static/flags/flag-france.png");
    img.set_id("language_button_image");
    switch_language_button.append_child(&img);
    return Ok(switch_language_button);
}
pub fn switch_language() {
    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    // let language: HtmlButtonElement = document
    //     .get_element_by_id("language_button")
    //     .unwrap()
    //     .dyn_into::<web_sys::HtmlButtonElement>()
    //     .unwrap();
    // let theme_cycling: HashMap<String, String> = hashmap![
    //     "static/flags/flag-france.png".to_owned()=>"static/flags/flag-united-kingdom.png".to_owned(),
    //     "static/flags/flag-united-kingdom.png".to_owned()=>"static/flags/flag-france.png".to_owned()
    // ];
    let language_cycling: HashMap<String, String> = hashmap![
        "fr".to_owned()=>"en".to_owned(),
        "en".to_owned()=>"fr".to_owned()
    ];
    let new_language = &language_cycling[&get_language()];
    set_language(new_language);
}
struct Language {
    key: String,
    long: String,
    flag: String,
}
pub fn set_language(language: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let flags: HashMap<String, String> = hashmap![
        "fr".to_owned()=>"flags/flag-france.png".to_owned(),
        "en".to_owned()=>"flags/flag-united-kingdom.png".to_owned()
    ];
    match document.get_element_by_id("language_button_image") {
        Some(language_button_image_wrapper) => {
            let language_button_image: HtmlImageElement = language_button_image_wrapper
                .dyn_into::<HtmlImageElement>()
                .unwrap();
            language_button_image.set_src(&flags[language]);
        }
        None => {
            error("language button not found");
        }
    };
    let storage: Storage = window.local_storage().unwrap().unwrap();
    storage.set_item("language", &language).unwrap();
}
pub fn get_language() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let storage: Storage = window.local_storage().unwrap().unwrap();
    match storage.get_item("language").unwrap() {
        Some(language) => language,
        None => "en".to_string(),
    }
}
pub fn get_language_full() -> String {
    return hashmap![
        "fr".to_owned()=>"french".to_owned(),
        "en".to_owned()=>"english".to_owned()
    ][&get_language()].to_string();
}
pub fn load_language() {
    set_language(&get_language());
}
