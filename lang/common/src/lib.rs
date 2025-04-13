use std::error::Error;
pub mod errors;
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
    type Err: Error;
    type Env;
    fn eval_start(self) -> Result<Self::Value, Self::Err>;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err>;
}

pub trait Typecheck<'a> {
    type Type;
    type Err: Error;
    type Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err>;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err>;
}

pub trait Parse: Sized {
    type Err: Error;
    fn parse(sourcte: String) -> Result<Self, Self::Err>;
}
