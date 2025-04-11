use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Lang {
    BoundedQuantification,
    Exceptions,
    Existential,
    Featherweight,
    FOmega,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lang::BoundedQuantification => f.write_str("Bounded Quantification"),
            Lang::Exceptions => f.write_str("Exceptions"),
            Lang::Existential => f.write_str("Existential"),
            Lang::Featherweight => f.write_str("Featherweight"),
            Lang::FOmega => f.write_str("FOmega"),
        }
    }
}
