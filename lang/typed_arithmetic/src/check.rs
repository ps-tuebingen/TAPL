use crate::{
    errors::Error,
    syntax::{Term, Type},
};

pub fn check(t: &Term) -> Result<Type, Error> {
    match t {
        Term::True => Ok(Type::Bool),
        Term::False => Ok(Type::Bool),
        Term::Zero => Ok(Type::Nat),
        Term::Succ(t) => {
            let ty = check(t)?;
            ty.check_equal(Type::Nat)?;
            Ok(Type::Nat)
        }
        Term::Pred(t) => {
            let ty = check(t)?;
            ty.check_equal(Type::Nat)?;
            Ok(Type::Nat)
        }
        Term::IsZero(t) => {
            let ty = check(t)?;
            ty.check_equal(Type::Nat)?;
            Ok(Type::Bool)
        }
        Term::If { ifc, thent, elset } => {
            let cond_ty = check(ifc)?;
            cond_ty.check_equal(Type::Bool)?;
            let then_ty = check(thent)?;
            let else_ty = check(elset)?;
            then_ty.check_equal(else_ty)?;
            Ok(then_ty)
        }
    }
}
