use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, HtmlTextAreaElement};

#[derive(Clone)]
struct HtmlContext {
    run_button: HtmlButtonElement,
    source_area: HtmlTextAreaElement,
}

impl HtmlContext {
    pub fn new() -> Rc<HtmlContext> {
        let doc = web_sys::window().unwrap().document().unwrap();
        let run_button = doc
            .get_element_by_id("run_button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        let source_area = doc
            .get_element_by_id("source_code")
            .unwrap()
            .dyn_into::<HtmlTextAreaElement>()
            .unwrap();
        Rc::new(Self {
            run_button,
            source_area,
        })
    }

    pub fn setup_events(ctx: Rc<Self>) {
        let ctx_ = ctx.clone();
        let button_handler =
            wasm_bindgen::closure::Closure::wrap(
                Box::new(move || ctx.handle_button()) as Box<dyn Fn()>
            );
        ctx_.run_button
            .add_event_listener_with_callback("click", button_handler.as_ref().unchecked_ref())
            .unwrap();
        button_handler.forget();
    }

    pub fn handle_button(&self) {
        let text_value = self.source_area.value();
        alert(&text_value);
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn setup() {
    let ctx = HtmlContext::new();
    HtmlContext::setup_events(ctx)
}
