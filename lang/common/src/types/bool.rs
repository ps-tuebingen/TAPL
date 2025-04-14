use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Bool;

impl Type for Bool {}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bool")
    }
}
