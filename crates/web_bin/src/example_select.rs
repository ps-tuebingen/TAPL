use super::{examples::all_examples, get_by_id};
use language::AllLanguages;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, HtmlOptionElement, HtmlSelectElement};

#[derive(Clone)]
pub struct ExampleSelect {
    pub element: HtmlSelectElement,
    examples: HashMap<&'static str, HashMap<&'static str, &'static str>>,
}

impl ExampleSelect {
    pub fn new(doc: &Document) -> ExampleSelect {
        let examples = all_examples();
        let element: HtmlSelectElement = get_by_id("example_select", doc);

        ExampleSelect { element, examples }
    }

    pub fn handle_change(&self, language: &AllLanguages, doc: &Document) {
        println!("changing to {language}");
        self.element.set_inner_html("");
        for (example_name, example_str) in self.examples.get(language.to_string().as_str()).unwrap()
        {
            let example_option = doc
                .create_element("option")
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();
            example_option.set_inner_html(example_name);
            self.element.append_child(&example_option).unwrap();
        }
    }
}
