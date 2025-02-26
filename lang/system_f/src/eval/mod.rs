pub mod errors;
pub mod lambda;
pub mod terms;
pub mod tylambda;
pub mod value;

use errors::Error;
pub use value::Value;

pub trait Eval {
    fn eval(self) -> Result<Value, Error>;
}
