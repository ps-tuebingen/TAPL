use crate::{example_select::ExampleSelect, get_by_id, out_divs::OutDivs, renderMathInElement};
use driver::{Driver, cli::Command, format::FormatMethod, languages::AllLanguages};
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{
    Document, HtmlButtonElement, HtmlOptionElement, HtmlSelectElement, HtmlTextAreaElement,
};

#[derive(Clone)]
pub struct HtmlContext {
    run_button: HtmlButtonElement,
    source_area: HtmlTextAreaElement,
    out_divs: OutDivs,
    example_select: ExampleSelect,
    lang_driver: Driver,
}

impl HtmlContext {
    pub fn new() -> Rc<HtmlContext> {
        let example_select = ExampleSelect::new(&document);
        let out_divs = OutDivs::new(&document);

        let run_button = get_by_id("run_button", &document);
        let source_area = get_by_id("source_code", &document);

        let lang_driver = Driver;

        let ctx = Rc::new(Self {
            document,
            run_button,
            source_area,
            out_divs,
            example_select,
            lang_driver,
        });
        ctx.setup_events();
        ctx
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
            self_.example_select.change_contents(&self_.get_lang());
        }) as Box<dyn Fn()>);
        self.example_select
            .element
            .add_event_listener_with_callback("change", change_handler.as_ref().unchecked_ref())
            .unwrap();
        change_handler.forget();
    }

    fn handle_button(&self) {
        let lang = self.get_lang();
        let source = self.source_area.value();
        self.out_divs.clear();
        let (parse_res, check_res, eval_res, err_res) =
            self.lang_driver
                .run_all_lang(source, &lang, &FormatMethod::LatexFracStripped);

        self.out_divs.parsed.set_contents(parse_res);
        self.out_divs.checked.set_contents(check_res);
        self.out_divs.evaled.set_contents(eval_res);
        self.out_divs.error.set_contents(err_res);
        renderMathInElement(&self.out_divs.parsed.out_div);
        renderMathInElement(&self.out_divs.checked.out_div);
        renderMathInElement(&self.out_divs.evaled.out_div);
        renderMathInElement(&self.out_divs.error.out_div);
    }
}
