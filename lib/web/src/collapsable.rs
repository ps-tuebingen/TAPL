use crate::{get_by_id, log, renderMathInElement};
use errors::{AddEventHandler, SetAttribute, web_error::WebError};
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{Document, Element, HtmlButtonElement};

pub struct CollapsableElement<T>
where
    T: AsRef<Element>,
{
    button: HtmlButtonElement,
    hide_element: T,
    elem_id: String,
    button_id: String,
}

impl<T> CollapsableElement<T>
where
    T: AsRef<Element>,
{
    pub fn new(
        doc: &Document,
        button_id: &str,
        element_id: &str,
    ) -> Result<Rc<CollapsableElement<T>>, WebError>
    where
        T: JsCast + 'static,
    {
        let button = get_by_id(button_id, doc)?;
        let hide_element = get_by_id(element_id, doc)?;
        let slf = Rc::new(CollapsableElement {
            button,
            hide_element,
            elem_id: element_id.to_owned(),
            button_id: button_id.to_owned(),
        });
        slf.clone().setup_events()?;
        Ok(slf)
    }

    pub fn hide(&self) -> Result<(), WebError> {
        self.hide_element
            .as_ref()
            .set_attribute("style", "display:none;")
            .map_err(|_| SetAttribute::new(&self.elem_id, "style", "display:none;"))?;
        self.hide_element
            .as_ref()
            .set_attribute("hidden", "true")
            .map_err(|_| SetAttribute::new(&self.elem_id, "hidden", "true"))?;
        Ok(())
    }

    pub fn show(&self) -> Result<(), WebError> {
        self.hide_element
            .as_ref()
            .set_attribute("style", "display:block;")
            .map_err(|_| SetAttribute::new(&self.elem_id, "style", "display:block;"))?;
        self.hide_element
            .as_ref()
            .set_attribute("hidden", "false")
            .map_err(|_| SetAttribute::new(&self.elem_id, "hidden", "false"))?;
        Ok(())
    }

    pub fn hidden(&self) -> bool {
        self.hide_element
            .as_ref()
            .get_attribute("hidden")
            .unwrap_or("true".to_owned())
            == "true"
    }

    pub fn toggle(&self) -> Result<(), WebError> {
        if self.hidden() {
            self.show()
        } else {
            self.hide()
        }
    }

    pub fn set_contents(&self, new_content: &str) -> Result<(), WebError> {
        self.hide_element.as_ref().set_inner_html(new_content);
        self.show()?;
        renderMathInElement(self.hide_element.as_ref());
        Ok(())
    }

    pub fn clear(&self) -> Result<(), WebError> {
        self.set_contents("")?;
        self.hide()?;
        Ok(())
    }

    pub fn setup_events(self: &Rc<Self>) -> Result<(), WebError>
    where
        T: 'static,
    {
        let self_ = self.clone();
        let button_handler = Closure::wrap(Box::new(move || match self_.toggle() {
            Ok(_) => return,
            Err(err) => log(&format!("{err}")),
        }) as Box<dyn Fn()>);
        self.button
            .add_event_listener_with_callback("click", button_handler.as_ref().unchecked_ref())
            .map_err(|_| AddEventHandler::new(&self.button_id, "click"))?;
        button_handler.forget();
        Ok(())
    }
}
