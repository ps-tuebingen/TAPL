use std::fmt;

/// Program did not contain a main definition
#[derive(Debug)]
pub struct UndefinedMain;

impl fmt::Display for UndefinedMain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("No main Function Defined")
    }
}
