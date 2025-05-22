use languages::AllLanguages;
use std::rc::Rc;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::{
    Document, HtmlButtonElement, HtmlDivElement, HtmlOptionElement, HtmlSelectElement,
    HtmlTextAreaElement,
};

#[derive(Clone)]
struct OutDivs {
    parsed: HtmlDivElement,
    checked: HtmlDivElement,
    evaled: HtmlDivElement,
    error: HtmlDivElement,
}

impl OutDivs {
    pub fn new(doc: &Document) -> OutDivs {
        let parsed = HtmlContext::get_by_id("parsed_out", doc);
        let checked = HtmlContext::get_by_id("checked_out", doc);
        let evaled = HtmlContext::get_by_id("evaled_out", doc);
        let error = HtmlContext::get_by_id("error_out", doc);
        OutDivs {
            parsed,
            checked,
            evaled,
            error,
        }
    }

    fn clear(&self) {
        self.parsed.set_inner_html("");
        self.checked.set_inner_html("");
        self.evaled.set_inner_html("");
        self.error.set_inner_html("");
    }
}

#[derive(Clone)]
struct HtmlContext {
    run_button: HtmlButtonElement,
    source_area: HtmlTextAreaElement,
    out_divs: OutDivs,
    language_select: HtmlSelectElement,
}

impl HtmlContext {
    pub fn new() -> Rc<HtmlContext> {
        let doc = web_sys::window().unwrap().document().unwrap();
        let run_button = Self::get_by_id("run_button", &doc);
        let source_area = Self::get_by_id("source_code", &doc);
        let out_divs = OutDivs::new(&doc);
        let language_select: HtmlSelectElement = Self::get_by_id("language_select", &doc);
        let ctx = Rc::new(Self {
            run_button,
            source_area,
            out_divs,
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

    fn get_lang(&self) -> AllLanguages {
        AllLanguages::all()[self.language_select.selected_index() as usize]
    }

    fn handle_button(&self) {
        let lang = self.get_lang();
        let source = self.source_area.value();
        self.out_divs.clear();
        lang.run(
            source,
            false,
            |p| self.out_divs.parsed.set_inner_html(p),
            |ty| self.out_divs.checked.set_inner_html(ty),
            |v| self.out_divs.evaled.set_inner_html(v),
            |err| self.out_divs.error.set_inner_html(err),
        );
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
