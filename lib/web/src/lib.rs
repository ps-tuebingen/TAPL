use driver::languages::AllLanguages;
use errors::{web_error::WebError, CouldNotCast, ElementNotFound};
use std::any::type_name;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::{Document, Element};

pub mod collapsable;
pub mod example_select;
pub mod language_select;
pub mod source_area;

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

pub fn get_lang(ind: usize) -> AllLanguages {
    AllLanguages::all()[ind]
}
