use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{NameMismatch, TypeKind, TypeMismatch};
use syntax::{
    env::Environment,
    terms::{Term, Unpack},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Unpack<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<Ty>,
    <T as Typecheck>::CheckError:
        From<TypeMismatch> + From<NameMismatch> + From<<Ty as Kindcheck<Ty>>::CheckError>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Type, Self::CheckError> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        if let Ok(bound_exists) = bound_ty.clone().into_exists() {
            if self.ty_name != bound_exists.var {
                return Err(NameMismatch::new(&bound_exists.var, &self.ty_name).into());
            }
            env.add_tyvar_kind(bound_exists.var, bound_exists.kind);
            env.add_var(self.term_name.clone(), *bound_exists.ty);
            Ok(self.in_term.check(&mut env.clone())?.normalize(env))
        } else if let Ok(bound_bound) = bound_ty.clone().into_exists_bounded() {
            if self.ty_name != bound_bound.var {
                return Err(NameMismatch::new(&bound_bound.var, &self.ty_name).into());
            }
            let sup_kind = bound_bound.sup_ty.check_kind(env)?;
            env.add_tyvar_super(bound_bound.var, *bound_bound.sup_ty.clone());
            env.add_tyvar_kind(self.ty_name.clone(), sup_kind);
            env.add_var(self.term_name.clone(), *bound_bound.ty.clone());
            let inner_ty = self.in_term.check(&mut env.clone())?;
            Ok(inner_ty.normalize(env))
        } else {
            Err(TypeMismatch::new(bound_ty.knd(), TypeKind::Existential).into())
        }
    }
}
