use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Document, HtmlElement};

mod context;
mod example_select;
mod out_divs;
#[rustfmt::skip]
mod examples;

use context::HtmlContext;

#[wasm_bindgen(start)]
pub fn setup() {
    HtmlContext::new();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(msg: &str);

    #[wasm_bindgen(js_namespace=window)]
    pub fn renderMathInElement(elem: &HtmlElement);

}

pub fn get_by_id<T>(id: &str, doc: &Document) -> T
where
    T: JsCast,
{
    doc.get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}
