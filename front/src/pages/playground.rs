//use crate::parts;
//use wasm_bindgen::prelude::*;
use crate::flow::code::flow_code;
//use crate::flow::code::MiroviaFlow;
use crate::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlDivElement;
use web_sys::HtmlTextAreaElement;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::Response;
 use crate::parts::top_tools::get_language_full;
struct Example<'a> {
    title: &'a str,
    content: String,
    id: &'a str,
}
pub async fn build(example_id: usize) -> Result<web_sys::HtmlDivElement, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let playground_div = document
        .create_element("div")?
        .dyn_into::<web_sys::HtmlDivElement>()?;
    playground_div.set_id("playground");
    // Left
    let left_div = document
        .create_element("div")?
        .dyn_into::<web_sys::HtmlDivElement>()?;
    left_div.set_id("left");
    playground_div.append_child(&left_div)?;
    // Example
    let examples_div = document
        .create_element("div")?
        .dyn_into::<web_sys::HtmlDivElement>()?;
    examples_div.set_id("examples");
    left_div.append_child(&examples_div)?;
    let examples_title = document
        .create_element("p")?
        .dyn_into::<web_sys::HtmlParagraphElement>()?;
    examples_title.set_inner_text("Examples");
    examples_div.set_id("examples");
    examples_div.append_child(&examples_title)?;
    let examples_list_div = document
        .create_element("div")?
        .dyn_into::<web_sys::HtmlDivElement>()?;
    examples_list_div.set_id("examples_list");
    examples_div.append_child(&examples_list_div)?;
    let examples: Vec<Example> = vec![
        Example {
            title: "Hello World",
            content: fetch_example("/examples/000_hello_world.gf")
                .await
                .unwrap()
                .to_string(),
            id: "hello",
        },
        Example {
            title: "Hello Someone",
            content: fetch_example("/examples/001_hello_someone.gf")
                .await
                .unwrap()
                .to_string(),
            id: "hello-someone",
        },
        // Example {
        //     title: "Calculator",
        //     content: fetch_example("examples/hello_world.gc").await.unwrap().to_string(),
        // },
        // Example {
        //     title: "Search engine",
        //     content: fetch_example("examples/hello_world.gc").await.unwrap().to_string(),
        // }
    ];
    for example in examples.iter() {
        let example_selecter = match document
            .create_element("p")?
            .dyn_into::<web_sys::HtmlParagraphElement>() {
                Ok(x)=>x,
                Err(e) => panic!("{} {} {:?}", file!(), line!(), e)
            };
        example_selecter.set_inner_text(&format!("- {}", example.title));
        example_selecter.set_id(&format!("example-{}", example.title));
        example_selecter
            .class_list()
            .add_1("playground_example_selecter")?;
        examples_list_div.append_child(&example_selecter)?;
        let content = example.content.clone();
        let id = example.id.clone();
        let open_example_trigger =
            Closure::wrap(Box::new(move || open_example(&content, &id)) as Box<dyn FnMut()>);
        example_selecter.set_onclick(Some(open_example_trigger.as_ref().unchecked_ref()));
        // TODO: do something with resulting JsValue
        open_example_trigger.into_js_value();
    }
    // Text
    let text_editor_div = document
        .create_element("div")?
        .dyn_into::<web_sys::HtmlDivElement>()?;
    text_editor_div.set_id("text_editor");
    left_div.append_child(&text_editor_div)?;
    let text_editor_textarea = document
        .create_element("textarea")?
        .dyn_into::<web_sys::HtmlTextAreaElement>()?;
    text_editor_textarea.set_id("text_editor_textarea");
    text_editor_div.append_child(&text_editor_textarea)?;
    // nodes
    let node_editor_div = document
        .create_element("div")?
        .dyn_into::<HtmlDivElement>()?;
    node_editor_div.set_id("node_editor");
    playground_div.append_child(&node_editor_div)?;
    open_example_inner(text_editor_textarea, node_editor_div, &examples[example_id].content)?;
    Ok(playground_div)
}
fn open_example_inner(
    text_editor_textarea: HtmlTextAreaElement,
    node_editor_div: HtmlDivElement,
    example_content: &String,
) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    // TODO: better reset
    node_editor_div.set_inner_html("");
    text_editor_textarea.set_value(example_content);
    let code = flow_code(example_content);

    // log(&format!("{:?}", code));

    let display_wrapper = document
        .create_element("div")?
        .dyn_into::<web_sys::HtmlDivElement>()?;
    display_wrapper.set_id(&"display_wrapper");
    node_editor_div.append_child(&display_wrapper)?;

    let function_wrapper = document
        .create_element("div")?
        .dyn_into::<web_sys::HtmlDivElement>()?;
    function_wrapper.set_id(&"function_wrapper");
    node_editor_div.append_child(&function_wrapper)?;

    let display_input_wrapper = document
        .create_element("div")?
        .dyn_into::<web_sys::HtmlDivElement>()?;
    display_input_wrapper.set_id(&"display_input_wrapper");
    node_editor_div.append_child(&display_input_wrapper)?;
    for (display_k, display_v) in code.display {
        let display_div = document
            .create_element("div")?
            .dyn_into::<web_sys::HtmlDivElement>()?;
        display_div.set_id(&format!("main.display.{}", display_k));
        display_div.set_class_name("display");
        let display_title = document
            .create_element("h1")?
            .dyn_into::<web_sys::HtmlHeadingElement>()?;
        display_title.set_inner_text(&display_v.title[&get_language_full()]);
        display_div.append_child(&display_title)?;
        let display_content = document
            .create_element("p")?
            .dyn_into::<web_sys::HtmlParagraphElement>()?;
        display_content.set_inner_text("...");
        display_div.append_child(&display_content)?;
        display_wrapper.append_child(&display_div)?;
    }
    for (function_k, function_v) in code.function {
        let function_div = document
            .create_element("div")?
            .dyn_into::<web_sys::HtmlDivElement>()?;
        let id = &format!("main.function.{}", function_k);
        function_div.set_id(id);
        function_div.set_class_name("function");
        let function_id = document
            .create_element("p")?
            .dyn_into::<web_sys::HtmlParagraphElement>()?;
        function_id.set_inner_text(id);
        function_div.append_child(&function_id)?;
        function_id.set_class_name("id");

        let function_content = document
            .create_element("p")?
            .dyn_into::<web_sys::HtmlParagraphElement>()?;
        function_content.set_inner_text(&function_v.output[&get_language_full()]);
        function_div.append_child(&function_content)?;
        function_content.set_class_name("content");
        function_wrapper.append_child(&function_div)?;
    }
    match code.display_input {
        Some(display_input) => {
            for (display_input_k, display_input_v) in display_input {
                log("zip");
                let display_input_div = document
                    .create_element("div")?
                    .dyn_into::<web_sys::HtmlDivElement>()?;
                display_input_div.set_id(&format!("main.display_input.{}", display_input_k));
                display_input_div.set_class_name("display_input");
                log("zip 1");
                let display_input_title = document
                    .create_element("h1")?
                    .dyn_into::<web_sys::HtmlHeadingElement>()?;
                display_input_title.set_inner_text(&display_input_v.title[&get_language_full()]);
                display_input_div.append_child(&display_input_title)?;
                log("zip 2");
                let display_input_content = document
                    .create_element("p")?
                    .dyn_into::<web_sys::HtmlParagraphElement>()?;
                display_input_content.set_inner_text("...");
                log("zip 3");
                display_input_div.append_child(&display_input_content)?;
                log("zip 4");
                display_input_wrapper.append_child(&display_input_div)?;
                log("zip 5s");
            }
        }
        None => {
            log("no display_input");
        }
    }
    Ok(())
}
// TODO: update browser url
fn open_example(example_content: &String, example_id: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let text_editor_textarea: HtmlTextAreaElement = document
        .get_element_by_id("text_editor_textarea")
        .unwrap()
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap();
    let node_editor_div: HtmlDivElement = document
        .get_element_by_id("node_editor")
        .unwrap()
        .dyn_into::<web_sys::HtmlDivElement>()
        .unwrap();
    open_example_inner(text_editor_textarea, node_editor_div, example_content).unwrap();
    let data: &JsValue = &JsValue::from_str("");
    let title: &str = "";
    let url = format!("/playground/{}", example_id);
    let url_wrapped: Option<&str> = Some(&url);
    window.history().unwrap().push_state_with_url(data, title, url_wrapped).unwrap();
}
pub async fn fetch_example(url: &str) -> Result<String, JsValue> {
    let opts = RequestInit::new();
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let x: String = JsFuture::from(resp.text()?).await?.as_string().unwrap();
    Ok(x)
}
