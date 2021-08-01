use crate::parts;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
struct Explanation<'a> {
    title: &'a str,
    text: &'a str,
    _more: &'a str,
}
pub fn go() -> Result<(), JsValue> {
    parts::top::add()?;
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    body.set_id("home_page_body");
    // let title = document
    //     .create_element("h1")?
    //     .dyn_into::<web_sys::HtmlHeadingElement>()?;
    // title.set_text_content(Some("Gouttelettes"));
    // body.append_child(&title)?;
    let p_open = document
        .create_element("p")?
        .dyn_into::<web_sys::HtmlParagraphElement>()?;
    p_open.set_text_content(Some(
        "Open, fair, unified platform for all people, all creations, with ease and peace of mind.",
    ));
    body.append_child(&p_open)?;
    let explanations = vec!(
        Explanation {
            title: "Open",
            text: "built on libre software, open to everyone",
            _more: ""
        },
        Explanation {
            title: "Fair",
            text: "set your own price, from free to infinity",
            _more: ""
        },
        Explanation {
            title: "Unified",
            text: "plugin system, all working with each other, be it frontend or backend",
            _more: ""
        },
        Explanation {
            title: "All people",
            text: "artists, makers, business people, your grandparents, rich or poor, technical or not",
            _more: ""
        },
        Explanation {
            title: "All creations",
            text: "business, art, tool, experience: think it, make it",
            _more: ""
        },
        Explanation {
            title: "Ease",
            text: "easy to use, hard too master",
            _more: ""
        },
        Explanation {
            title: "Peace of mind",
            text: "robust by essence, autoscaling",
            _more: "Built on top of Rust, with serverless autoscaling architecture"
        },
    );
    for x in explanations.iter() {
        // let h2_p = document
        //     .create_element("h2")?
        //     .dyn_into::<web_sys::HtmlHeadingElement>()?;
        // h2_p.set_text_content(Some(&x.title));
        // body.append_child(&h2_p)?;
        let p_open = document
            .create_element("p")?
            .dyn_into::<web_sys::HtmlParagraphElement>()?;
        p_open.set_text_content(Some(&format!("• {}: {}", &x.title, &x.text)));
        body.append_child(&p_open)?;
    }
    Ok(())
}
