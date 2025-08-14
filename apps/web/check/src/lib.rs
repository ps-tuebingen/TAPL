use errors::web_error::WebError;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen};
use web::{
    example_select::ExampleSelect, get_lang, language_select::LanguageSelect, log,
    source_area::SourceArea,
};

struct CheckContext {
    language_select: LanguageSelect,
    example_select: ExampleSelect,
    source_area: SourceArea,
}

impl CheckContext {
    pub fn new() -> Result<Rc<CheckContext>, WebError> {
        let window = web_sys::window().ok_or(WebError::Window)?;
        let document = window.document().ok_or(WebError::Document)?;
        let language_select = LanguageSelect::new(&document)?;
        let example_select = ExampleSelect::new(&document)?;
        let source_area = SourceArea::new(&document)?;
        let slf = Rc::new(CheckContext {
            language_select,
            example_select,
            source_area,
        });
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
                    return;
                }
            };
            match self_.set_example() {
                Ok(_) => (),
                Err(err) => {
                    log(&format!("{err}"));
                    return;
                }
            }
        }) as Box<dyn Fn()>);

        let self_ = self.clone();
        let change_handler_example = Closure::wrap(Box::new(move || match self_.set_example() {
            Ok(_) => return,
            Err(err) => {
                log(&format!("{err}"));
                return;
            }
        }) as Box<dyn Fn()>);

        self.language_select.setup_events(change_handler_language)?;
        self.example_select.setup_events(change_handler_example)?;
        Ok(())
    }

    fn set_example(&self) -> Result<(), WebError> {
        let example = self.example_select.get_example()?;
        self.source_area.set_contents(&example);
        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    match CheckContext::new() {
        Ok(_) => return,
        Err(err) => log(&format!("{err}")),
    }
}
