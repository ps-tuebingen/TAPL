use std::fmt;

/// Error while creating html element
#[derive(Debug)]
pub struct CreateElement {
    /// html tag of the element
    ty: String,
}

impl CreateElement {
    /// Create a new error from the tag
    #[must_use]
    pub fn new(ty: &str) -> Self {
        Self { ty: ty.to_owned() }
    }
}

impl fmt::Display for CreateElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not create {} element", self.ty)
    }
}

impl std::error::Error for CreateElement {}
