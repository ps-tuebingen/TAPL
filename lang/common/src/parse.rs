use crate::errors::Error;

pub trait Parse: Sized {
    fn parse(sourcte: String) -> Result<Self, Error>;
}
