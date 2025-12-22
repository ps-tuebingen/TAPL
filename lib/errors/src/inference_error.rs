use std::fmt;

#[derive(Debug)]
pub enum InferenceError {}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for InferenceError {}
