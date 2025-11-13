use std::fmt;

#[derive(Debug)]
pub struct AppendChild {
    parent_id: String,
    child_id: String,
}

impl AppendChild {
    #[must_use] pub fn new(parent: &str, child: &str) -> Self {
        Self {
            parent_id: parent.to_owned(),
            child_id: child.to_owned(),
        }
    }
}

impl fmt::Display for AppendChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not append child {} to {}",
            self.child_id, self.parent_id
        )
    }
}

impl std::error::Error for AppendChild {}
