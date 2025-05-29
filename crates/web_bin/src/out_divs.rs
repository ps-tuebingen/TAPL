use super::get_by_id;
use web_sys::{Document, HtmlDivElement};

#[derive(Clone)]
pub struct OutDivs {
    pub parsed: HtmlDivElement,
    pub checked: HtmlDivElement,
    pub evaled: HtmlDivElement,
    pub error: HtmlDivElement,
}

impl OutDivs {
    pub fn new(doc: &Document) -> OutDivs {
        let parsed = get_by_id("parsed_out", doc);
        let checked = get_by_id("checked_out", doc);
        let evaled = get_by_id("evaled_out", doc);
        let error = get_by_id("error_out", doc);
        OutDivs {
            parsed,
            checked,
            evaled,
            error,
        }
    }

    pub fn clear(&self) {
        self.parsed.set_inner_html("");
        self.checked.set_inner_html("");
        self.evaled.set_inner_html("");
        self.error.set_inner_html("");
    }
}
