use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::errors::{KindMismatch, TypeKind, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    subst::SubstType,
    terms::{Pack, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Pack<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty> + Subtypecheck<Ty> + SubstType<Ty, Target = Ty>,
    <T as Typecheck>::CheckError: From<TypeMismatch>
        + From<KindMismatch>
        + From<<Ty as Kindcheck<Ty>>::CheckError>
        + From<<Ty as Subtypecheck<Ty>>::CheckError>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let outer_norm = self.outer_ty.clone().normalize(env.clone());
        let inner_kind = self.inner_ty.check_kind(env.clone())?;
        let outer_knd = outer_norm.check_kind(env.clone())?;

        if let Ok(outer_exists) = outer_norm.clone().into_exists() {
            env.add_tyvar_kind(outer_exists.var.clone(), outer_exists.kind.clone());
            let term_res = self.term.check(env.clone())?;
            let term_ty = term_res.ty().normalize(env.clone());
            let term_kind = term_ty.check_kind(env.clone())?;

            term_kind.check_equal(&outer_knd)?;
            inner_kind.check_equal(&outer_exists.kind)?;

            let outer_subst = outer_exists
                .ty
                .subst_type(&outer_exists.var, &self.inner_ty)
                .normalize(env.clone());
            outer_subst.check_equal(&term_ty)?;

            let conc = Conclusion::new(env, self.clone(), self.outer_ty.clone());
            let deriv = Derivation::pack(conc, term_res);
            Ok(deriv)
        } else if let Ok(outer_bound) = outer_norm.clone().into_exists_bounded() {
            let sup_norm = outer_bound.sup_ty.clone().normalize(env.clone());
            let sup_kind = sup_norm.check_kind(env.clone())?;
            env.add_tyvar_super(outer_bound.var.clone(), *outer_bound.sup_ty.clone());
            env.add_tyvar_kind(outer_bound.var.clone(), sup_kind);

            let term_res = self.term.check(env.clone())?;
            let term_ty = term_res.ty();
            let term_kind = term_ty.check_kind(env.clone())?;
            term_kind.check_equal(&outer_knd)?;

            let outer_subst = outer_bound.ty.subst_type(&outer_bound.var, &self.inner_ty);
            term_ty.check_subtype(&outer_subst, env.clone())?;
            let conc = Conclusion::new(env, self.clone(), self.outer_ty.clone());
            let deriv = Derivation::pack_bound(conc, term_res);

            Ok(deriv)
        } else {
            Err(TypeMismatch::new(outer_norm.knd(), TypeKind::Existential).into())
        }
    }
}
