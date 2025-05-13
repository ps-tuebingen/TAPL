use common::{eval::Eval, parse::Parse};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, HtmlDivElement, HtmlTextAreaElement};

#[derive(Clone)]
struct HtmlContext {
    run_button: HtmlButtonElement,
    source_area: HtmlTextAreaElement,
    parsed_out: HtmlDivElement,
    evaled_out: HtmlDivElement,
    error_out: HtmlDivElement,
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
        let parsed_out = doc
            .get_element_by_id("parsed_out")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        let evaled_out = doc
            .get_element_by_id("evaled_out")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        let error_out = doc
            .get_element_by_id("errors")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        let ctx = Rc::new(Self {
            run_button,
            source_area,
            parsed_out,
            evaled_out,
            error_out,
        });
        Self::setup_events(&ctx);
        ctx
    }

    pub fn setup_events(ctx: &Rc<Self>) {
        let ctx_ = ctx.clone();
        let button_handler =
            wasm_bindgen::closure::Closure::wrap(
                Box::new(move || ctx_.handle_button()) as Box<dyn Fn()>
            );
        ctx.run_button
            .add_event_listener_with_callback("click", button_handler.as_ref().unchecked_ref())
            .unwrap();
        button_handler.forget();
    }

    pub fn set_err<T>(&self, err: T)
    where
        T: std::error::Error,
    {
        self.error_out.set_inner_html(&err.to_string());
    }

    pub fn reset_err(&self) {
        self.error_out.set_inner_html("");
    }

    pub fn handle_button(&self) {
        let text_value = self.source_area.value();
        let parsed = match untyped_lambda::terms::Term::parse(text_value) {
            Ok(p) => p,
            Err(err) => return self.set_err(err),
        };
        self.parsed_out.set_inner_html(&parsed.to_string());
        let evaled = match parsed.eval_start() {
            Ok(e) => e,
            Err(err) => return self.set_err(err),
        };
        self.evaled_out.set_inner_html(&evaled.to_string());
        self.reset_err();
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn setup() {
    HtmlContext::new();
}
