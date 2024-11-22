use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Unit;

impl From<Unit> for Term {
    fn from(unit: Unit) -> Term {
        Term::Unit(unit)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unit")
    }
}
