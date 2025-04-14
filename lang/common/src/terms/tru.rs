use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct True;

impl Term for True {}

impl fmt::Display for True {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}
