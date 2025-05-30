use super::{examples::all_examples, get_by_id, log};
use language::{languages::UntypedArithmetic, AllLanguages};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlOptionElement, HtmlSelectElement, HtmlTextAreaElement};

#[derive(Clone)]
pub struct ExampleSelect {
    pub element: HtmlSelectElement,
    examples: HashMap<&'static str, Vec<(&'static str, &'static str)>>,
    source_area: HtmlTextAreaElement,
}

impl ExampleSelect {
    pub fn new(doc: &Document) -> ExampleSelect {
        let examples = all_examples();
        let element: HtmlSelectElement = get_by_id("example_select", doc);
        let source_area: HtmlTextAreaElement = get_by_id("source_code", doc);

        let slf = ExampleSelect {
            element,
            examples,
            source_area,
        };
        slf.change_language(&UntypedArithmetic.into(), doc);
        slf
    }

    pub fn change_language(&self, language: &AllLanguages, doc: &Document) {
        log(&format!("changing to {language}"));
        self.element.set_inner_html("");
        for (example_name, _) in self.examples.get(language.to_string().as_str()).unwrap() {
            let example_option = doc
                .create_element("option")
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();
            example_option.set_inner_html(example_name);
            self.element.append_child(&example_option).unwrap();
        }
        self.change_contents(&language);
    }

    pub fn change_contents(&self, language: &AllLanguages) {
        let language_examples = self.examples.get(language.to_string().as_str()).unwrap();
        let (_, current_example) = language_examples[self.element.selected_index() as usize];
        self.source_area.set_inner_html(current_example);
    }
}
