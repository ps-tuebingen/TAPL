use crate::{
    errors::{Error, ErrorKind},
    terms::Var,
};

pub mod bool;
pub mod lambda;
pub mod nat;
pub mod pack;
pub mod record;
pub mod term;
pub mod value;
pub use value::Value;

pub trait Eval {
    fn eval(self) -> Result<Value, Error>;
}

impl Eval for Var {
    fn eval(self) -> Result<Value, Error> {
        Err(Error::eval(ErrorKind::var(&self), &self.as_str()))
    }
}
