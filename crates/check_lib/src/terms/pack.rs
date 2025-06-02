use crate::{CheckEnvironment, Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::errors::{KindMismatch, TypeKind, TypeMismatch};
use syntax::{
    subst::SubstType,
    terms::{Pack, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Pack<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>
        + Subtypecheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>
        + SubstType<Ty, Target = Ty>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let outer_norm = self.outer_ty.clone().normalize(env);
        let inner_kind = self.inner_ty.check_kind(env)?;
        let outer_knd = outer_norm.check_kind(env)?;

        if let Ok(outer_exists) = outer_norm.clone().into_exists() {
            env.add_tyvar_kind(outer_exists.var.clone(), outer_exists.kind.clone());
            let term_ty = self.term.check(env)?.normalize(env);
            let term_kind = term_ty.check_kind(env)?;

            term_kind.check_equal(&outer_knd)?;
            inner_kind.check_equal(&outer_exists.kind)?;

            let outer_subst = dbg!(outer_exists
                .ty
                .subst_type(&outer_exists.var, &self.inner_ty))
            .normalize(env);
            outer_subst.check_equal(&term_ty)?;
            Ok(self.outer_ty.clone())
        } else if let Ok(outer_bound) = outer_norm.clone().into_exists_bounded() {
            let sup_norm = outer_bound.sup_ty.clone().normalize(env);
            let sup_kind = sup_norm.check_kind(env)?;
            env.add_tyvar_super(outer_bound.var.clone(), *outer_bound.sup_ty.clone());
            env.add_tyvar_kind(outer_bound.var.clone(), sup_kind);

            let term_ty = self.term.check(env)?;
            let term_kind = term_ty.check_kind(env)?;
            term_kind.check_equal(&outer_knd)?;

            let outer_subst = outer_bound.ty.subst_type(&outer_bound.var, &self.inner_ty);
            term_ty.check_subtype(&outer_subst, env)?;
            Ok(self.outer_ty.clone())
        } else {
            Err(TypeMismatch::new(outer_norm.knd(), TypeKind::Existential).into())
        }
    }
}
