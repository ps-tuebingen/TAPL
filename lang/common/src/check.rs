use crate::errors::Error;

pub trait Typecheck<'a> {
    type Type;
    type Env;
    fn check_start(&self) -> Result<Self::Type, Error>;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error>;
}
