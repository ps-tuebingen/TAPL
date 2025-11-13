use std::fmt;

#[derive(Debug)]
pub struct TriggerEvent {
    event: String,
}

impl TriggerEvent {
    #[must_use] pub fn new(event: &str) -> Self {
        Self {
            event: event.to_owned(),
        }
    }
}

impl fmt::Display for TriggerEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not trigger event {}", self.event)
    }
}

impl std::error::Error for TriggerEvent {}
