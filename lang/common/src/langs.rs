use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Lang {
    BoundedQuantification,
    Exceptions,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lang::BoundedQuantification => f.write_str("Bounded Quantification"),
            Lang::Exceptions => f.write_str("Exceptions"),
        }
    }
}
