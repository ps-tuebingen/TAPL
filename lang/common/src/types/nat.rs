use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Nat;

impl Type for Nat {}

impl fmt::Display for Nat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Nat")
    }
}
