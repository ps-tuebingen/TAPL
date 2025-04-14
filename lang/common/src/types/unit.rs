use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Unit;

impl Type for Unit {}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Unit")
    }
}
