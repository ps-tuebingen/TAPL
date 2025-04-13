use crate::{
    syntax::{Term, Type},
    to_err,
};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Typecheck,
};

pub fn to_check_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Check)
}

impl Typecheck<'_> for Term {
    type Type = Type;
    type Env = ();
    type Err = Error;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(())
    }

    fn check(&self, _env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Term::True => Ok(Type::Bool),
            Term::False => Ok(Type::Bool),
            Term::Zero => Ok(Type::Nat),
            Term::Succ(t) => {
                let ty = t.check(_env)?;
                ty.check_equal(Type::Nat).map_err(to_check_err)?;
                Ok(Type::Nat)
            }
            Term::Pred(t) => {
                let ty = t.check(_env)?;
                ty.check_equal(Type::Nat).map_err(to_check_err)?;
                Ok(Type::Nat)
            }
            Term::IsZero(t) => {
                let ty = t.check(_env)?;
                ty.check_equal(Type::Nat).map_err(to_check_err)?;
                Ok(Type::Bool)
            }
            Term::If { ifc, thent, elset } => {
                let cond_ty = ifc.check(_env)?;
                cond_ty.check_equal(Type::Bool).map_err(to_check_err)?;
                let then_ty = thent.check(_env)?;
                let else_ty = elset.check(_env)?;
                then_ty.check_equal(else_ty).map_err(to_check_err)?;
                Ok(then_ty)
            }
        }
    }
}
