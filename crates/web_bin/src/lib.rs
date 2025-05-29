use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::Document;

mod context;
mod example_select;
mod examples;
mod out_divs;

use context::HtmlContext;

#[wasm_bindgen(start)]
pub fn setup() {
    HtmlContext::new();
}

pub fn get_by_id<T>(id: &str, doc: &Document) -> T
where
    T: JsCast,
{
    doc.get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}
