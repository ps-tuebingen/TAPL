use crate::{examples::all_examples, get_by_id, log};
use errors::{
    AddEventHandler, AppendChild, CouldNotCast, CreateElement, GetAttribute, SetAttribute,
    TriggerEvent, UndefinedExample, UndefinedLanguage, web_error::WebError,
};
use std::collections::HashMap;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{Document, Event, HtmlOptionElement, HtmlSelectElement};

#[derive(Clone)]
pub struct ExampleSelect {
    document: Document,
    id: String,
    element: HtmlSelectElement,
    examples: HashMap<&'static str, Vec<(&'static str, &'static str)>>,
}

impl ExampleSelect {
    pub fn new(doc: &Document, start_lang: &str) -> Result<ExampleSelect, WebError> {
        let id = "example_select".to_owned();
        let element = get_by_id(&id, doc)?;
        let examples = all_examples();
        let slf = ExampleSelect {
            document: doc.clone(),
            id,
            element,
            examples,
        };
        slf.set_options(start_lang)?;
        Ok(slf)
    }

    pub fn set_options(&self, language: &str) -> Result<(), WebError> {
        self.element
            .set_attribute("language", language)
            .map_err(|_| SetAttribute::new(&self.id, "language", language))?;
        self.element.set_inner_html("");
        let options = self
            .examples
            .get(language)
            .ok_or(UndefinedLanguage::new(&language))?;
        for (example_name, _) in options {
            let option_elem = self
                .document
                .create_element("option")
                .map_err(|_| CreateElement::new("option"))?
                .dyn_into::<HtmlOptionElement>()
                .map_err(|_| CouldNotCast::new(&example_name, "option"))?;
            option_elem.set_inner_html(example_name);
            self.element
                .append_child(&option_elem)
                .map_err(|_| AppendChild::new(&self.id, &example_name))?;
        }
        Ok(())
    }

    pub fn setup_events(&self, change_handler: Closure<dyn Fn()>) -> Result<(), WebError> {
        self.element
            .add_event_listener_with_callback("change", change_handler.as_ref().unchecked_ref())
            .map_err(|_| AddEventHandler::new(&self.id, "change"))?;
        change_handler.forget();
        Ok(())
    }

    pub fn get_example(&self) -> Result<String, WebError> {
        let lang = self
            .element
            .get_attribute("language")
            .ok_or(GetAttribute::new(&self.id, "language"))?;
        let example_ind = self.element.selected_index() as usize;
        let lang_examples = self
            .examples
            .get(lang.as_str())
            .ok_or(UndefinedLanguage::new(&lang))?;
        let (_, example) = lang_examples
            .get(example_ind)
            .ok_or(UndefinedExample::new(example_ind))?;
        Ok((*example).to_owned())
    }
}
