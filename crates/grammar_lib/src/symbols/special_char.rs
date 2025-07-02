use super::Symbol;

#[derive(Clone)]
pub enum SpecialChar {
    Lambda,
    Mu,
    Exclamation,
    Forall,
    Exists,
}

impl From<SpecialChar> for Symbol {
    fn from(sc: SpecialChar) -> Symbol {
        Symbol::SpecialChar(sc)
    }
}
