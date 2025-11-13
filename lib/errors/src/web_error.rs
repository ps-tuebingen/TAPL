use crate::{
    AddEventHandler, AppendChild, CouldNotCast, CreateElement, ElementNotFound, GetAttribute,
    SetAttribute, TriggerEvent, UndefinedExample, UndefinedLanguage,
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
    GetAttribute(GetAttribute),
    UndefinedExample(UndefinedExample),
    UndefinedLanguage(UndefinedLanguage),
    TriggerEvent(TriggerEvent),
    Window,
    Document,
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ElementNotFound(enf) => enf.fmt(f),
            Self::CouldNotCast(cnc) => cnc.fmt(f),
            Self::CreateElement(ce) => ce.fmt(f),
            Self::AppendChild(ac) => ac.fmt(f),
            Self::AddEventHandler(aeh) => aeh.fmt(f),
            Self::SetAttribute(sa) => sa.fmt(f),
            Self::GetAttribute(ga) => ga.fmt(f),
            Self::UndefinedLanguage(ul) => ul.fmt(f),
            Self::UndefinedExample(ue) => ue.fmt(f),
            Self::TriggerEvent(te) => te.fmt(f),
            Self::Window => write!(f, "Could not get window element"),
            Self::Document => write!(f, "Could not get html document"),
        }
    }
}

impl std::error::Error for WebError {}

impl From<ElementNotFound> for WebError {
    fn from(err: ElementNotFound) -> Self {
        Self::ElementNotFound(err)
    }
}

impl From<CouldNotCast> for WebError {
    fn from(err: CouldNotCast) -> Self {
        Self::CouldNotCast(err)
    }
}

impl From<CreateElement> for WebError {
    fn from(err: CreateElement) -> Self {
        Self::CreateElement(err)
    }
}

impl From<AppendChild> for WebError {
    fn from(err: AppendChild) -> Self {
        Self::AppendChild(err)
    }
}

impl From<AddEventHandler> for WebError {
    fn from(err: AddEventHandler) -> Self {
        Self::AddEventHandler(err)
    }
}

impl From<SetAttribute> for WebError {
    fn from(err: SetAttribute) -> Self {
        Self::SetAttribute(err)
    }
}

impl From<UndefinedLanguage> for WebError {
    fn from(err: UndefinedLanguage) -> Self {
        Self::UndefinedLanguage(err)
    }
}

impl From<GetAttribute> for WebError {
    fn from(err: GetAttribute) -> Self {
        Self::GetAttribute(err)
    }
}

impl From<UndefinedExample> for WebError {
    fn from(err: UndefinedExample) -> Self {
        Self::UndefinedExample(err)
    }
}

impl From<TriggerEvent> for WebError {
    fn from(err: TriggerEvent) -> Self {
        Self::TriggerEvent(err)
    }
}
