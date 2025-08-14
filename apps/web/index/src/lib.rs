use driver::format::FormatMethod;
use errors::web_error::WebError;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen};
use web::{collapsable::CollapsableElement, get_lang, language_select::LanguageSelect, log};
use web_sys::HtmlDivElement;
//mod context;
//mod example_select;
//mod out_divs;
//use context::HtmlContext;

struct IndexContext {
    language_select: LanguageSelect,
    grammar_out: Rc<CollapsableElement<HtmlDivElement>>,
}

impl IndexContext {
    fn new() -> Result<Rc<IndexContext>, WebError> {
        let window = web_sys::window().ok_or(WebError::Window)?;
        let document = window.document().ok_or(WebError::Document)?;
        let language_select = LanguageSelect::new(&document)?;
        let grammar_out =
            CollapsableElement::new(&document, "grammar_collapse", "grammar_out").unwrap();

        let slf = Rc::new(IndexContext {
            language_select,
            grammar_out,
        });
        slf.grammar_out.set_contents(&slf.get_grammar())?;
        slf.grammar_out.show()?;
        slf.clone().setup_events()?;
        Ok(slf)
    }

    fn get_grammar(&self) -> String {
        FormatMethod::LatexFracStripped
            .format(&get_lang(self.language_select.selected()).grammars())
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
                Ok(_) => return,
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
        Ok(_) => return,
        Err(err) => log(&format!("{err}")),
    }
}
