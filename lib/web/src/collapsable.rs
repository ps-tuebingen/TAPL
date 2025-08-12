use crate::get_by_id;
use errors::web_error::WebError;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlButtonElement};

pub struct CollapsableElement<T> {
    button: HtmlButtonElement,
    hide_element: T,
}

impl<T> CollapsableElement<T> {
    pub fn new(
        doc: &Document,
        button_id: &str,
        element_id: &str,
    ) -> Result<CollapsableElement<T>, WebError>
    where
        T: JsCast,
    {
        let button = get_by_id(button_id, doc)?;
        let hide_element = get_by_id(element_id, doc)?;
        Ok(CollapsableElement {
            button,
            hide_element,
        })
    }
}
