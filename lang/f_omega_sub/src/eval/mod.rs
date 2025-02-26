use crate::errors::Error;

pub mod terms;
pub mod types;
pub mod value;
use crate::Env;
pub use value::Value;

pub trait Eval {
    type Target;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error>;
}
