use driver::Driver;
use errors::web_error::WebError;
use languages::dispatch::{Command, FormatMethod};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen};
use web::{
    collapsable::CollapsableElement, language_select::LanguageSelect, log, web_langs::WEB_LANGUAGES,
};
use web_sys::HtmlDivElement;

struct IndexContext {
    language_select: LanguageSelect,
    grammar_out: Rc<CollapsableElement<HtmlDivElement>>,
    driver: RefCell<Driver>,
}

impl IndexContext {
    fn new() -> Result<Rc<IndexContext>, WebError> {
        let window = web_sys::window().ok_or(WebError::Window)?;
        let document = window.document().ok_or(WebError::Document)?;
        let language_select = LanguageSelect::new(&document, false)?;
        let grammar_out =
            CollapsableElement::new(&document, "grammar_collapse", "grammar_out").unwrap();
        let driver = RefCell::new(Driver::new());
        let slf = Rc::new(IndexContext {
            language_select,
            grammar_out,
            driver,
        });
        slf.grammar_out.set_contents(&slf.get_grammar())?;
        slf.grammar_out.show()?;
        slf.clone().setup_events()?;
        Ok(slf)
    }

    fn get_grammar(&self) -> String {
        let lang = WEB_LANGUAGES[self.language_select.selected()];
        self.driver
            .borrow_mut()
            .run_command(
                "".into(),
                lang,
                Command::Grammar,
                FormatMethod::LatexFracStripped,
            )
            .unwrap()
    }

    fn setup_events(self: Rc<Self>) -> Result<(), WebError> {
        let self_ = self.clone();
        let change_handler = Closure::wrap(Box::new(move || {
            match self_.grammar_out.clear() {
                Ok(_) => (),
                Err(err) => {
                    log(&format!("{err}"));
                    return;
                }
            }
            let res = self_.grammar_out.set_contents(&self_.get_grammar());
            match res {
                Ok(_) => (),
                Err(err) => log(&format!("{err}")),
            }
        }) as Box<dyn Fn()>);
        self.language_select.setup_events(change_handler)?;
        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    match IndexContext::new() {
        Ok(_) => (),
        Err(err) => log(&format!("{err}")),
    }
}
