use crate::{
    AddEventHandler, AppendChild, CouldNotCast, CreateElement, ElementNotFound, SetAttribute,
};
use std::fmt;

#[derive(Debug)]
pub enum WebError {
    ElementNotFound(ElementNotFound),
    CouldNotCast(CouldNotCast),
    CreateElement(CreateElement),
    AppendChild(AppendChild),
    AddEventHandler(AddEventHandler),
    SetAttribute(SetAttribute),
    Window,
    Document,
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebError::ElementNotFound(enf) => enf.fmt(f),
            WebError::CouldNotCast(cnc) => cnc.fmt(f),
            WebError::CreateElement(ce) => ce.fmt(f),
            WebError::AppendChild(ac) => ac.fmt(f),
            WebError::AddEventHandler(aeh) => aeh.fmt(f),
            WebError::SetAttribute(sa) => sa.fmt(f),
            WebError::Window => write!(f, "Could not get window element"),
            WebError::Document => write!(f, "Could not get html document"),
        }
    }
}

impl std::error::Error for WebError {}

impl From<ElementNotFound> for WebError {
    fn from(err: ElementNotFound) -> WebError {
        WebError::ElementNotFound(err)
    }
}

impl From<CouldNotCast> for WebError {
    fn from(err: CouldNotCast) -> WebError {
        WebError::CouldNotCast(err)
    }
}

impl From<CreateElement> for WebError {
    fn from(err: CreateElement) -> WebError {
        WebError::CreateElement(err)
    }
}

impl From<AppendChild> for WebError {
    fn from(err: AppendChild) -> WebError {
        WebError::AppendChild(err)
    }
}

impl From<AddEventHandler> for WebError {
    fn from(err: AddEventHandler) -> WebError {
        WebError::AddEventHandler(err)
    }
}

impl From<SetAttribute> for WebError {
    fn from(err: SetAttribute) -> WebError {
        WebError::SetAttribute(err)
    }
}
