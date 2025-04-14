use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Top;

impl Type for Top {}

impl fmt::Display for Top {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Top")
    }
}
