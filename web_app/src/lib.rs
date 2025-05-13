use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let doc = web_sys::window().unwrap().document().unwrap();

    let run_button = doc.get_element_by_id("run_button").unwrap();
    let button_callback =
        wasm_bindgen::closure::Closure::wrap(Box::new(handle_button) as Box<dyn Fn()>);
    run_button
        .add_event_listener_with_callback("click", button_callback.as_ref().unchecked_ref())
        .unwrap();
    button_callback.forget();
}

pub fn handle_button() {
    let doc = web_sys::window().unwrap().document().unwrap();
    let code_area = doc
        .get_element_by_id("source_code")
        .unwrap()
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap();
    alert(&code_area.value())
}
