use super::Type;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Bot;

impl Type for Bot {}

impl fmt::Display for Bot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bot")
    }
}
