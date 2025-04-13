use super::EvalContext;
use common::errors::Error;
use common::Var;

pub mod ascription;
pub mod bool;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod nat;
pub mod optional;
pub mod pair;
pub mod record;
pub mod sum;
pub mod term;
pub mod tup;
pub mod unit;
pub mod variant;

pub trait AsContext {
    fn to_context(self) -> Result<EvalContext, Error>;
}

impl AsContext for Var {
    fn to_context(self) -> Result<EvalContext, Error> {
        Ok(EvalContext::Var(self))
    }
}
