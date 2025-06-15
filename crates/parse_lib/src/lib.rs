pub mod errors;

pub trait Parse: Sized {
    type Rule;
    type ParseError: std::error::Error + From<pest::error::Error<Self::Rule>>;
    fn parse(sourcte: String) -> Result<Self, Self::ParseError>;
}
