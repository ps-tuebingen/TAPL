use crate::errors::Error;

pub mod app;
pub mod lambda;
pub mod pack;
pub mod record;
pub mod record_proj;
pub mod terms;
pub mod tyapp;
pub mod tylambda;
pub mod unpack;
pub mod value;
pub use value::Value;

pub trait Eval {
    fn eval(self) -> Result<Value, Error>;
}
