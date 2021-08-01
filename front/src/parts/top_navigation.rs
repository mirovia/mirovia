use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlAnchorElement;
use web_sys::HtmlImageElement;
use web_sys::HtmlInputElement;
use web_sys::HtmlSpanElement;
pub fn add() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let nav_div = document.create_element("div")?;
    nav_div.set_id("nav_div");
    body.append_child(&nav_div)?;
    let home_link: HtmlAnchorElement = document
        .create_element("a")?
        .dyn_into::<web_sys::HtmlAnchorElement>()?;
    home_link.set_href("/");
    let icon: HtmlImageElement = document
        .create_element("img")?
        .dyn_into::<web_sys::HtmlImageElement>()?;
    icon.set_src("/favicon.svg");
    home_link.append_child(&icon)?;
    let home_link_text: HtmlSpanElement = document
        .create_element("span")?
        .dyn_into::<web_sys::HtmlSpanElement>()?;
    home_link_text.set_inner_text("Gouttelettes");
    home_link.append_child(&home_link_text)?;
    nav_div.append_child(&home_link)?;
    let playground_link: HtmlAnchorElement = document
        .create_element("a")?
        .dyn_into::<web_sys::HtmlAnchorElement>()?;
    playground_link.set_inner_text("Playground");
    playground_link.set_href("/playground");
    nav_div.append_child(&playground_link)?;
    let create_account: HtmlAnchorElement = document
        .create_element("a")?
        .dyn_into::<web_sys::HtmlAnchorElement>()?;
    create_account.set_inner_text("Create account");
    create_account.set_href("/enter");
    nav_div.append_child(&create_account)?;
    let spacer: HtmlSpanElement = document
        .create_element("span")?
        .dyn_into::<web_sys::HtmlSpanElement>()?;
    spacer.set_class_name("flex-spacer");
    nav_div.append_child(&spacer)?;
    let username_email: HtmlInputElement = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    username_email.set_placeholder("Username or Email");
    nav_div.append_child(&username_email)?;
    let password: HtmlInputElement = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    password.set_placeholder("Password");
    password.set_type("password");
    nav_div.append_child(&password)?;
    let button_login = document.create_element("button")?;
    button_login.set_text_content(Some("Login"));
    nav_div.append_child(&button_login)?;
    Ok(())
}
