use super::{get_by_id, log};
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Document, HtmlButtonElement, HtmlDivElement};

#[derive(Clone, Copy, Debug)]
pub enum OutDivName {
    Parsed,
    Checked,
    Evaled,
    Error,
}

impl OutDivName {
    pub fn div_id(&self) -> &str {
        match self {
            OutDivName::Parsed => "parsed_out",
            OutDivName::Checked => "checked_out",
            OutDivName::Evaled => "evaled_out",
            OutDivName::Error => "error_out",
        }
    }

    pub fn button_id(&self) -> &str {
        match self {
            OutDivName::Parsed => "parsed_collapse",
            OutDivName::Checked => "checked_collapse",
            OutDivName::Evaled => "evaled_collapse",
            OutDivName::Error => "error_collapse",
        }
    }
}

#[derive(Clone)]
pub struct OutDiv {
    out_div: HtmlDivElement,
    collapse_button: HtmlButtonElement,
}

impl OutDiv {
    pub fn new(name: OutDivName, doc: &Document) -> Rc<OutDiv> {
        log(&format!("creating {name:?}"));
        let out_div = get_by_id(name.div_id(), doc);
        log("got div");
        let collapse_button = get_by_id(name.button_id(), doc);
        log("got button");
        let slf = Rc::new(OutDiv {
            out_div,
            collapse_button,
        });
        log("got outdiv and collapse button");
        slf.hide();
        log("hidden");
        slf.setup_events();
        log("setup events");
        slf
    }

    pub fn hide(&self) {
        self.out_div
            .set_attribute("style", "display:none;")
            .expect("Could not set display");
        self.out_div
            .set_attribute("hidden", "true")
            .expect("Could not set attribute");
    }

    pub fn show(&self) {
        self.out_div
            .set_attribute("style", "display:block;")
            .expect("Could not set display");
        self.out_div
            .set_attribute("hidden", "false")
            .expect("Could not set attribute");
    }

    pub fn toggle(&self) {
        let hidden = self
            .out_div
            .get_attribute("hidden")
            .unwrap_or("true".to_owned());
        if &hidden == "true" {
            self.show()
        } else {
            self.hide()
        }
    }

    pub fn clear(&self) {
        self.out_div.set_inner_html("");
    }

    pub fn set_contents(&self, new_content: Option<String>) {
        if let Some(s) = new_content {
            self.out_div.set_inner_html(&format!("\\[{s}\\]"));
        }
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
}

impl OutDivs {
    pub fn new(doc: &Document) -> OutDivs {
        let parsed = OutDiv::new(OutDivName::Parsed, doc);
        log("created parsed");
        let checked = OutDiv::new(OutDivName::Checked, doc);
        log("created checked");
        let evaled = OutDiv::new(OutDivName::Evaled, doc);
        log("created evaled");
        let error = OutDiv::new(OutDivName::Error, doc);
        log("created error");

        OutDivs {
            parsed,
            checked,
            evaled,
            error,
        }
    }

    pub fn clear(&self) {
        self.parsed.clear();
        self.checked.clear();
        self.evaled.clear();
        self.error.clear();
    }
}
