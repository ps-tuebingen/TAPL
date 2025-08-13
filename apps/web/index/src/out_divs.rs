use super::{get_by_id, log};
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{Document, HtmlButtonElement, HtmlDivElement};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutDivName {
    Parsed,
    Checked,
    Evaled,
    Error,
    Grammar,
}

impl OutDivName {
    pub fn div_id(&self) -> &str {
        match self {
            OutDivName::Parsed => "parsed_out",
            OutDivName::Checked => "checked_out",
            OutDivName::Evaled => "evaled_out",
            OutDivName::Error => "error_out",
            OutDivName::Grammar => "grammar_out",
        }
    }

    pub fn button_id(&self) -> &str {
        match self {
            OutDivName::Parsed => "parsed_collapse",
            OutDivName::Checked => "checked_collapse",
            OutDivName::Evaled => "evaled_collapse",
            OutDivName::Error => "error_collapse",
            OutDivName::Grammar => "grammar_collapse",
        }
    }
}

#[derive(Clone)]
pub struct OutDiv {
    pub out_div: HtmlDivElement,
    collapse_button: HtmlButtonElement,
    which: OutDivName,
}

impl OutDiv {
    pub fn new(name: OutDivName, doc: &Document) -> Rc<OutDiv> {
        let out_div = get_by_id(name.div_id(), doc);
        let collapse_button = get_by_id(name.button_id(), doc);
        let slf = Rc::new(OutDiv {
            out_div,
            collapse_button,
            which: name,
        });
        slf.hide();
        slf.setup_events();
        slf
    }

    pub fn setup_events(self: &Rc<Self>) {
        let self_ = self.clone();
        let button_handler = Closure::wrap(Box::new(move || self_.toggle()) as Box<dyn Fn()>);
        self.collapse_button
            .add_event_listener_with_callback("click", button_handler.as_ref().unchecked_ref())
            .unwrap();
        button_handler.forget();
    }
}

#[derive(Clone)]
pub struct OutDivs {
    pub parsed: Rc<OutDiv>,
    pub checked: Rc<OutDiv>,
    pub evaled: Rc<OutDiv>,
    pub error: Rc<OutDiv>,
    pub grammar: Rc<OutDiv>,
}

impl OutDivs {
    pub fn new(doc: &Document) -> OutDivs {
        let parsed = OutDiv::new(OutDivName::Parsed, doc);
        let checked = OutDiv::new(OutDivName::Checked, doc);
        let evaled = OutDiv::new(OutDivName::Evaled, doc);
        let error = OutDiv::new(OutDivName::Error, doc);
        let grammar = OutDiv::new(OutDivName::Grammar, doc);

        OutDivs {
            parsed,
            checked,
            evaled,
            error,
            grammar,
        }
    }

    pub fn clear(&self) {
        self.parsed.clear();
        self.checked.clear();
        self.evaled.clear();
        self.error.clear();
    }
}
