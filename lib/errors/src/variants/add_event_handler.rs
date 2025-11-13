use std::fmt;

#[derive(Debug)]
pub struct AddEventHandler {
    id: String,
    event: String,
}

impl AddEventHandler {
    #[must_use] pub fn new(id: &str, event: &str) -> Self {
        Self {
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
