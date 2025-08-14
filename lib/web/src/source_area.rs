use crate::get_by_id;
use errors::web_error::WebError;
use web_sys::{Document, HtmlTextAreaElement};

pub struct SourceArea {
    id: String,
    element: HtmlTextAreaElement,
}

impl SourceArea {
    pub fn new(doc: &Document) -> Result<SourceArea, WebError> {
        let id = "source_code".to_owned();
        let element = get_by_id(&id, &doc)?;
        Ok(SourceArea { id, element })
    }

    pub fn set_contents(&self, contents: &str) {
        self.element.set_inner_html(contents)
    }

    pub fn get_contents(&self) -> String {
        self.element.value()
    }
}
