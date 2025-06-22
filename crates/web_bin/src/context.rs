use super::{example_select::ExampleSelect, get_by_id, out_divs::OutDivs, typeset};
use language::{AllLanguages, FormatMethod};
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{
    Document, HtmlButtonElement, HtmlOptionElement, HtmlSelectElement, HtmlTextAreaElement,
};
#[derive(Clone)]
pub struct HtmlContext {
    document: Document,
    run_button: HtmlButtonElement,
    source_area: HtmlTextAreaElement,
    out_divs: OutDivs,
    example_select: ExampleSelect,
    language_select: HtmlSelectElement,
}

impl HtmlContext {
    pub fn new() -> Rc<HtmlContext> {
        let document = web_sys::window().unwrap().document().unwrap();

        let example_select = ExampleSelect::new(&document);
        let out_divs = OutDivs::new(&document);

        let run_button = get_by_id("run_button", &document);
        let source_area = get_by_id("source_code", &document);
        let language_select: HtmlSelectElement = get_by_id("language_select", &document);

        let ctx = Rc::new(Self {
            document,
            run_button,
            source_area,
            out_divs,
            language_select,
            example_select,
        });
        ctx.setup_languages();
        ctx.setup_events();
        ctx
    }

    fn setup_languages(&self) {
        for lang in AllLanguages::all() {
            let lang_option = self
                .document
                .create_element("option")
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();
            lang_option.set_id(&lang.to_string());
            lang_option.set_inner_html(lang.describe());
            self.language_select.append_child(&lang_option).unwrap();
        }
    }

    pub fn setup_events(self: &Rc<Self>) {
        let self_ = self.clone();
        let button_handler =
            Closure::wrap(Box::new(move || self_.handle_button()) as Box<dyn Fn()>);
        self.run_button
            .add_event_listener_with_callback("click", button_handler.as_ref().unchecked_ref())
            .unwrap();
        button_handler.forget();

        let self_ = self.clone();
        let change_handler = Closure::wrap(Box::new(move || {
            self_
                .example_select
                .change_language(&self_.get_lang(), &self_.document)
        }) as Box<dyn Fn()>);
        self.language_select
            .add_event_listener_with_callback("change", change_handler.as_ref().unchecked_ref())
            .unwrap();
        change_handler.forget();

        let self_ = self.clone();
        let change_handler =
            Closure::wrap(
                Box::new(move || self_.example_select.change_contents(&self_.get_lang()))
                    as Box<dyn Fn()>,
            );
        self.example_select
            .element
            .add_event_listener_with_callback("change", change_handler.as_ref().unchecked_ref())
            .unwrap();
        change_handler.forget();
    }

    fn get_lang(&self) -> AllLanguages {
        AllLanguages::all()[self.language_select.selected_index() as usize]
    }

    fn handle_button(&self) {
        let lang = self.get_lang();
        let source = self.source_area.value();
        self.out_divs.clear();
        let (parse_res, check_res, eval_res, err_res) =
            lang.run_all(source, &FormatMethod::LatexFrac);

        self.out_divs.parsed.set_contents(parse_res);
        self.out_divs.checked.set_contents(check_res);
        self.out_divs.evaled.set_contents(eval_res);
        self.out_divs.error.set_contents(err_res);
        typeset();
    }
}
