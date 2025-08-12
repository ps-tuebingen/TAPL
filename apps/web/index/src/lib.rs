use wasm_bindgen::prelude::wasm_bindgen;
use web::{collapsable::CollapsableElement, language_select::LanguageSelect};
use web_sys::{Document, HtmlDivElement};
//mod context;
//mod example_select;
//mod out_divs;
//use context::HtmlContext;

struct IndexContext {
    document: Document,
    language_select: LanguageSelect,
    grammar_out: CollapsableElement<HtmlDivElement>,
}

impl IndexContext {
    fn new() -> IndexContext {
        let document = web_sys::window().unwrap().document().unwrap();
        let language_select = LanguageSelect::new(&document).unwrap();
        let grammar_out =
            CollapsableElement::new(&document, "grammar_collapse", "grammar_out").unwrap();
        IndexContext {
            document,
            language_select,
            grammar_out,
        }
    }
}

#[wasm_bindgen(start)]
pub fn setup() {
    IndexContext::new();
}
