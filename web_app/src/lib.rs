use common::{eval::Eval, parse::Parse};
use languages::AllLanguages;
use std::rc::Rc;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::{
    Document, HtmlButtonElement, HtmlDivElement, HtmlOptionElement, HtmlSelectElement,
    HtmlTextAreaElement,
};

#[derive(Clone)]
struct HtmlContext {
    run_button: HtmlButtonElement,
    source_area: HtmlTextAreaElement,
    parsed_out: HtmlDivElement,
    evaled_out: HtmlDivElement,
    error_out: HtmlDivElement,
    language_select: HtmlSelectElement,
}

impl HtmlContext {
    pub fn new() -> Rc<HtmlContext> {
        let doc = web_sys::window().unwrap().document().unwrap();
        let run_button = Self::get_by_id("run_button", &doc);
        let source_area = Self::get_by_id("source_code", &doc);
        let parsed_out = Self::get_by_id("parsed_out", &doc);
        let evaled_out = Self::get_by_id("evaled_out", &doc);
        let error_out = Self::get_by_id("errors", &doc);
        let language_select = Self::get_by_id("language_select", &doc);
        let ctx = Rc::new(Self {
            run_button,
            source_area,
            parsed_out,
            evaled_out,
            error_out,
            language_select,
        });
        ctx.setup_languages(&doc);
        Self::setup_events(&ctx);
        ctx
    }

    fn get_by_id<T>(id: &str, doc: &Document) -> T
    where
        T: JsCast,
    {
        doc.get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
    }

    fn setup_languages(&self, doc: &Document) {
        for lang in AllLanguages::all() {
            let lang_option = doc
                .create_element("option")
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();
            lang_option.set_id(&lang.to_string());
            lang_option.set_inner_html(lang.describe());
            self.language_select.append_child(&lang_option).unwrap();
        }
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

    fn set_err<T>(&self, err: T)
    where
        T: std::error::Error,
    {
        self.error_out.set_inner_html(&err.to_string());
    }

    fn reset_err(&self) {
        self.error_out.set_inner_html("");
    }

    fn get_lang(&self) -> AllLanguages {
        AllLanguages::all()[self.language_select.selected_index() as usize]
    }

    fn handle_button(&self) {
        let lang = self.get_lang();
        alert(&lang.to_string());
        let text_value = self.source_area.value();
        let parsed = match languages::untyped_lambda::terms::Term::parse(text_value) {
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
