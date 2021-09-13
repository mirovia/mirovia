#[macro_use]
mod macros;
mod flow;
mod pages;
mod parts;
use parts::top_tools::load_language;
use parts::top_tools::load_theme;
use url::Url;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::Window;
#[wasm_bindgen]
pub async fn go() -> Result<(), JsValue> {
    log("Mirovia frontend running");
    set_panic_hook();
    load_theme();
    let window: Window = web_sys::window().expect("no global `window` exists");
    let url = window.location().href()?;
    log(&url);
    let parsed_url = Url::parse(&url).unwrap();
    match parsed_url.path() {
        "/" => pages::home::go(),
        "/enter" => pages::enter::go(),
        "/forgot" => pages::forgot_credentials::go(),
        "/playground" => pages::build(pages::playground::build(0).await),
        "/playground/hello" => pages::build(pages::playground::build(0).await),
        "/playground/hello-someone" => pages::build(pages::playground::build(1).await),
        // "/playground/hello-someone" => pages::home::go(),
        _ => pages::not_found::go(),
    }?;
    log("donee");
    load_language();
    Ok(())
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`.
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
