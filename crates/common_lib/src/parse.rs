pub trait Parse: Sized {
    type ParseErr: std::error::Error;
    fn parse(sourcte: String) -> Result<Self, Self::ParseErr>;
}
