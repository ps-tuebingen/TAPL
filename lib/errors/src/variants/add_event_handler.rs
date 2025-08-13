use std::fmt;

#[derive(Debug)]
pub struct AddEventHandler {
    id: String,
    event: String,
}

impl AddEventHandler {
    pub fn new(id: &str, event: &str) -> AddEventHandler {
        AddEventHandler {
            id: id.to_owned(),
            event: event.to_owned(),
        }
    }
}

impl fmt::Display for AddEventHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not add event handler for {} to {}",
            self.event, self.id
        )
    }
}

impl std::error::Error for AddEventHandler {}
