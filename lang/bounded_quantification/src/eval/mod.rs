use crate::errors::Error;

pub mod lambda;
pub mod lambda_sub;
pub mod nat;
pub mod pack;
pub mod term;
pub mod value;
pub use value::Value;

pub trait Eval {
    fn eval(self) -> Result<Value, Error>;
}
