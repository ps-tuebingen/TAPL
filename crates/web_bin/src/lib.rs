use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::Document;

mod context;
mod example_select;
#[rustfmt::skip]
mod examples;
mod out_divs;

use context::HtmlContext;

#[wasm_bindgen(start)]
pub fn setup() {
    HtmlContext::new();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(msg: &str);

    #[wasm_bindgen(js_namespace=MathJax)]
    pub fn typeset();
}

pub fn get_by_id<T>(id: &str, doc: &Document) -> T
where
    T: JsCast,
{
    doc.get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}
