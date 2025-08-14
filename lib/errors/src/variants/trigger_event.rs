use std::fmt;

#[derive(Debug)]
pub struct TriggerEvent {
    event: String,
}

impl TriggerEvent {
    pub fn new(event: &str) -> TriggerEvent {
        TriggerEvent {
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
