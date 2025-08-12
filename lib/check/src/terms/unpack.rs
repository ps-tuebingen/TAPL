use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use errors::{NameMismatch, TypeMismatch};
use syntax::{
    env::Environment,
    terms::{Term, Unpack},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Unpack<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Ty: TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<Ty>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Deriv, CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ty().normalize(env.clone());
        if let Ok(bound_exists) = bound_ty.clone().into_exists() {
            if self.ty_name != bound_exists.var {
                return Err(NameMismatch::new(&bound_exists.var, &self.ty_name).into());
            }
            env.add_tyvar_kind(bound_exists.var, bound_exists.kind);
            env.add_var(self.term_name.clone(), *bound_exists.ty);
            let in_res = self.in_term.check(env.clone())?;
            let in_ty = in_res.ty().normalize(env.clone());
            let conc = Conclusion::new(env, self.clone(), in_ty);
            let deriv = TypingDerivation::unpack(conc, bound_res, in_res);
            Ok(deriv)
        } else if let Ok(bound_bound) = bound_ty.clone().into_exists_bounded() {
            if self.ty_name != bound_bound.var {
                return Err(NameMismatch::new(&bound_bound.var, &self.ty_name).into());
            }
            let sup_kind = bound_bound.sup_ty.check_kind(env.clone())?;
            env.add_tyvar_super(bound_bound.var, *bound_bound.sup_ty.clone());
            env.add_tyvar_kind(self.ty_name.clone(), sup_kind);
            env.add_var(self.term_name.clone(), *bound_bound.ty.clone());
            let inner_res = self.in_term.check(env.clone())?;
            let inner_ty = inner_res.ty().normalize(env.clone());
            let conc = Conclusion::new(env.clone(), self.clone(), inner_ty);
            let deriv = TypingDerivation::unpack_bounded(conc, bound_res, inner_res);
            Ok(deriv)
        } else {
            Err(TypeMismatch::new(bound_ty.to_string(), "Existential Type".to_owned()).into())
        }
    }
}
