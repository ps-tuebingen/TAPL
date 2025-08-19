use driver::{Driver, cli::Command, format::FormatMethod};
use errors::{AddEventHandler, web_error::WebError};
use std::rc::Rc;
use wasm_bindgen::{
    closure::Closure,
    prelude::{JsCast, wasm_bindgen},
};
use web::{
    collapsable::CollapsableElement, example_select::ExampleSelect, get_by_id, get_lang,
    language_select::LanguageSelect, log, source_area::SourceArea,
};
use web_sys::{HtmlButtonElement, HtmlDivElement};

struct CheckContext {
    language_select: LanguageSelect,
    example_select: ExampleSelect,
    source_area: SourceArea,
    run_button: HtmlButtonElement,
    driver: Driver,
    eval_out: Rc<CollapsableElement<HtmlDivElement>>,
    error_out: Rc<CollapsableElement<HtmlDivElement>>,
}

impl CheckContext {
    pub fn new() -> Result<Rc<CheckContext>, WebError> {
        let window = web_sys::window().ok_or(WebError::Window)?;
        let document = window.document().ok_or(WebError::Document)?;
        let language_select = LanguageSelect::new(&document, false)?;
        let example_select =
            ExampleSelect::new(&document, &get_lang(language_select.selected()).to_string())?;
        let source_area = SourceArea::new(&document)?;
        let eval_out = CollapsableElement::new(&document, "eval_collapse", "eval_out")?;
        let error_out = CollapsableElement::new(&document, "error_collapse", "error_out")?;
        let run_button = get_by_id("run_button", &document)?;
        let driver = Driver;
        let slf = Rc::new(CheckContext {
            language_select,
            example_select,
            source_area,
            run_button,
            driver,
            eval_out,
            error_out,
        });
        slf.eval_out.hide()?;
        slf.error_out.hide()?;
        slf.clone().setup_events()?;
        slf.set_example()?;
        Ok(slf)
    }

    fn setup_events(self: Rc<Self>) -> Result<(), WebError> {
        let self_ = self.clone();
        let change_handler_language = Closure::wrap(Box::new(move || {
            let res = self_
                .example_select
                .set_options(&get_lang(self_.language_select.selected()).to_string());
            match res {
                Ok(_) => (),
                Err(err) => {
                    log(&format!("{err}"));
                }
            };
            match self_.set_example() {
                Ok(_) => (),
                Err(err) => {
                    log(&format!("{err}"));
                }
            }
        }) as Box<dyn Fn()>);

        let self_ = self.clone();
        let change_handler_example = Closure::wrap(Box::new(move || match self_.set_example() {
            Ok(_) => (),
            Err(err) => {
                log(&format!("{err}"));
            }
        }) as Box<dyn Fn()>);

        let self_ = self.clone();
        let button_handler = Closure::wrap(Box::new(move || match self_.eval_source() {
            Ok(_) => (),
            Err(err) => {
                log(&format!("{err}"));
            }
        }) as Box<dyn Fn()>);

        self.run_button
            .add_event_listener_with_callback("click", button_handler.as_ref().unchecked_ref())
            .map_err(|_| AddEventHandler::new("run_button", "click"))?;
        button_handler.forget();

        self.language_select.setup_events(change_handler_language)?;
        self.example_select.setup_events(change_handler_example)?;
        Ok(())
    }

    fn set_example(&self) -> Result<(), WebError> {
        let example = self.example_select.get_example()?;
        self.source_area.set_contents(&example);
        Ok(())
    }

    fn eval_source(&self) -> Result<(), WebError> {
        let source = self.source_area.get_contents();
        let lang = get_lang(self.language_select.selected());
        match self.driver.run_lang(
            source,
            &lang,
            &Command::Evaluate,
            &FormatMethod::LatexFracStripped,
        ) {
            Ok(ty) => {
                self.eval_out.set_contents(&ty)?;
                self.error_out.clear()?;
                Ok(())
            }
            Err(err) => {
                self.eval_out.clear()?;
                self.error_out.set_contents(&err.to_string())?;
                Ok(())
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    match CheckContext::new() {
        Ok(_) => (),
        Err(err) => log(&format!("{err}")),
    }
}
