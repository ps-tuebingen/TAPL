use crate::{
    errors::Error,
    syntax::{Term, Type},
};
use common::Typecheck;

impl Typecheck<'_> for Term {
    type Type = Type;
    type Env = ();
    type Err = Error;
    fn check(&self, _env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Term::True => Ok(Type::Bool),
            Term::False => Ok(Type::Bool),
            Term::Zero => Ok(Type::Nat),
            Term::Succ(t) => {
                let ty = t.check(_env)?;
                ty.check_equal(Type::Nat)?;
                Ok(Type::Nat)
            }
            Term::Pred(t) => {
                let ty = t.check(_env)?;
                ty.check_equal(Type::Nat)?;
                Ok(Type::Nat)
            }
            Term::IsZero(t) => {
                let ty = t.check(_env)?;
                ty.check_equal(Type::Nat)?;
                Ok(Type::Bool)
            }
            Term::If { ifc, thent, elset } => {
                let cond_ty = ifc.check(_env)?;
                cond_ty.check_equal(Type::Bool)?;
                let then_ty = thent.check(_env)?;
                let else_ty = elset.check(_env)?;
                then_ty.check_equal(else_ty)?;
                Ok(then_ty)
            }
        }
    }
}
