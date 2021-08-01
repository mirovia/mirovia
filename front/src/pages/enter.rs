use crate::log;
use crate::parts;
use wasm_bindgen::closure::Closure;
/*use wasm_bindgen::prelude::*;*/
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlAnchorElement;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
pub fn go() -> Result<(), JsValue> {
    parts::top::add()?;

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    body.set_id("enter_page_body");

    let home_page_forms_wrapper = document.create_element("div")?;
    home_page_forms_wrapper.set_id("home_page_forms_wrapper");
    body.append_child(&home_page_forms_wrapper)?;

    let login_form = document.create_element("div")?;
    login_form.set_id("login_form");
    home_page_forms_wrapper.append_child(&login_form)?;
    let login_title = document
        .create_element("h1")?
        .dyn_into::<web_sys::HtmlHeadingElement>()?;
    login_title.set_text_content(Some("Log in using an existing account"));
    login_form.append_child(&login_title)?;
    let username_email: HtmlInputElement = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    username_email.set_placeholder("Username or Email");
    login_form.append_child(&username_email)?;
    let password: HtmlInputElement = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    password.set_placeholder("Password");
    password.set_type("password");
    login_form.append_child(&password)?;
    let button_login = document.create_element("button")?;
    button_login.set_text_content(Some("Login"));
    login_form.append_child(&button_login)?;
    let forgot_password_link: HtmlAnchorElement = document
        .create_element("a")?
        .dyn_into::<web_sys::HtmlAnchorElement>()?;
    forgot_password_link.set_text_content(Some("Forgot password ?"));
    forgot_password_link.set_href("forgot");
    login_form.append_child(&forgot_password_link)?;

    let button_login_onclick = Closure::wrap(Box::new(move || login()) as Box<dyn FnMut()>);
    button_login
        .dyn_ref::<HtmlElement>()
        .expect("button_login is HtmlElement")
        .set_onclick(Some(button_login_onclick.as_ref().unchecked_ref()));
    // Todo; try not to use forget
    button_login_onclick.forget();

    let register_form = document.create_element("div")?;
    register_form.set_id("login_form");
    home_page_forms_wrapper.append_child(&register_form)?;

    let register_title = document
        .create_element("h1")?
        .dyn_into::<web_sys::HtmlHeadingElement>()?;
    register_title.set_text_content(Some("Or create a new account"));
    register_form.append_child(&register_title)?;

    let register_username: HtmlInputElement = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    register_username.set_placeholder("Username");
    register_form.append_child(&register_username)?;
    let register_password: HtmlInputElement = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    register_password.set_placeholder("Password");
    register_password.set_type("password");
    register_form.append_child(&register_password)?;

    let register_password_confirm: HtmlInputElement = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    register_password_confirm.set_placeholder("Confirm password");
    register_password_confirm.set_type("password");
    register_form.append_child(&register_password_confirm)?;

    let button_register = document.create_element("button")?;
    button_register.set_text_content(Some("Create account"));
    register_form.append_child(&button_register)?;

    Ok(())
}

pub fn login() {
    log("login");
}
