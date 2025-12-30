use crate::{
    get_by_id,
    web_langs::{WEB_LANGUAGES, WEB_LANGUAGES_TYPED},
};
use errors::{
    AddEventHandler, AppendChild, CouldNotCast, CreateElement, language_error::LanguageError,
    web_error::WebError,
};
use languages::dispatch::{DispatchLanguage, create_dispatcher};
use wasm_bindgen::{closure::Closure, prelude::JsCast};
use web_sys::{Document, HtmlOptionElement, HtmlSelectElement};

#[derive(Clone)]
pub struct LanguageSelect {
    document: Document,
    id: String,
    element: HtmlSelectElement,
}

impl LanguageSelect {
    pub fn new(doc: &Document, typed: bool) -> Result<LanguageSelect, WebError> {
        let id = "language_select".to_owned();
        let elem = get_by_id(&id, doc)?;
        let slf = LanguageSelect {
            id,
            document: doc.clone(),
            element: elem,
        };
        slf.setup_languages(typed)?;
        Ok(slf)
    }

    fn setup_languages(&self, typed: bool) -> Result<(), WebError> {
        let lang_ids = if typed {
            WEB_LANGUAGES_TYPED.as_slice()
        } else {
            WEB_LANGUAGES.as_slice()
        };
        let langs = lang_ids
            .iter()
            .map(|lang| create_dispatcher(lang))
            .collect::<Result<Vec<Box<dyn DispatchLanguage>>, LanguageError>>()?;
        for lang in langs {
            if typed && lang.id().to_lowercase().contains("untyped") {
                continue;
            }
            let child_id = lang.id();
            let lang_option = self
                .document
                .create_element("option")
                .map_err(|_| CreateElement::new("option"))?
                .dyn_into::<HtmlOptionElement>()
                .map_err(|_| CouldNotCast::new(&child_id, "option"))?;
            lang_option.set_id(&child_id);
            lang_option.set_inner_html(lang.describe());
            self.element
                .append_child(&lang_option)
                .map_err(|_| AppendChild::new(&self.id, &child_id))?;
        }
        Ok(())
    }

    pub fn setup_events(&self, change_handler: Closure<dyn Fn()>) -> Result<(), WebError> {
        self.element
            .add_event_listener_with_callback("change", change_handler.as_ref().unchecked_ref())
            .map_err(|_| AddEventHandler::new(&self.id, "change"))?;
        change_handler.forget();
        Ok(())
    }

    pub fn selected(&self) -> usize {
        self.element.selected_index() as usize
    }
}
