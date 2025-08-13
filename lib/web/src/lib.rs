use errors::{CouldNotCast, ElementNotFound, web_error::WebError};
use std::any::type_name;
use wasm_bindgen::{JsCast, prelude::wasm_bindgen};
use web_sys::{Document, Element};

pub mod collapsable;
pub mod language_select;

#[rustfmt::skip]
mod examples;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(msg: &str);

    #[wasm_bindgen(js_namespace=window)]
    pub fn renderMathInElement(elem: &Element);

}

pub fn get_by_id<T>(id: &str, doc: &Document) -> Result<T, WebError>
where
    T: JsCast,
{
    let elem = doc.get_element_by_id(id).ok_or(ElementNotFound::new(id))?;
    elem.dyn_into::<T>()
        .map_err(|_| CouldNotCast::new(id, type_name::<T>()).into())
}
