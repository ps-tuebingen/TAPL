use crate::errors::Error;

pub mod terms;
pub mod types;
use crate::Env;
pub use types::subtype::check_subtype;

pub trait Check {
    type Target;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error>;
}
