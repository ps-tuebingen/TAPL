use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Unit {}

impl Term for Unit {}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unit")
    }
}
