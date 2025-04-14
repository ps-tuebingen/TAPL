use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct False;

impl Term for False {}

impl fmt::Display for False {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}
