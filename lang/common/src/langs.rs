use std::fmt;

#[derive(Debug)]
pub enum Lang {
    BoundedQuantification,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lang::BoundedQuantification => f.write_str("Bounded Quantification"),
        }
    }
}
