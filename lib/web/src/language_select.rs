use crate::get_by_id;
use errors::web_error::WebError;
use web_sys::{Document, HtmlSelectElement};

pub struct LanguageSelect {
    elem: HtmlSelectElement,
}

impl LanguageSelect {
    pub fn new(doc: &Document) -> Result<LanguageSelect, WebError> {
        let elem = get_by_id("language_select", &doc)?;
        Ok(LanguageSelect { elem })
    }
}
