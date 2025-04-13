pub mod errors;
use errors::Error;
pub mod langs;

pub type Var = String;
pub type TypeVar = String;
pub type Label = String;
pub type Kind = String;
pub type Term = String;
pub type Location = usize;
pub type Type = String;
pub type Value = String;

pub trait Eval<'a> {
    type Value;
    type Env;
    fn eval_start(self) -> Result<Self::Value, Error>;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Error>;
}

pub trait Typecheck<'a> {
    type Type;
    type Env;
    fn check_start(&self) -> Result<Self::Type, Error>;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error>;
}

pub trait Parse: Sized {
    fn parse(sourcte: String) -> Result<Self, Error>;
}
