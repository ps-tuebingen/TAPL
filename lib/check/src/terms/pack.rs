use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::TypeMismatch;
use errors::check_error::CheckError;
use syntax::{
    env::Environment, language::Language, subst::SubstType, terms::Pack, types::TypeGroup,
};

impl<Lang> Typecheck for Pack<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang>
        + Normalize<Lang = Lang>
        + Kindcheck<Lang = Lang>
        + Subtypecheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let outer_norm = self.outer_ty.clone().normalize(env.clone());
        let inner_kind = self.inner_ty.check_kind(env.clone())?;
        let outer_knd = outer_norm.check_kind(env.clone())?;

        if let Ok(outer_exists) = outer_norm.clone().into_exists() {
            env.add_tyvar_kind(outer_exists.var.clone(), outer_exists.kind.clone());
            let term_res = self.term.check(env.clone())?;
            let term_ty = term_res.ret_ty().normalize(env.clone());
            let term_kind = term_ty.check_kind(env.clone())?;

            term_kind.check_equal(&outer_knd)?;
            inner_kind.check_equal(&outer_exists.kind)?;

            let outer_subst = outer_exists
                .ty
                .subst_type(&outer_exists.var, &self.inner_ty)
                .normalize(env.clone());
            outer_subst.check_equal(&term_ty)?;

            let conc = TypingConclusion::new(env, self.clone(), self.outer_ty.clone());
            let deriv = TypingDerivation::pack(conc, term_res);
            Ok(deriv.into())
        } else if let Ok(outer_bound) = outer_norm.clone().into_exists_bounded() {
            let sup_norm = outer_bound.sup_ty.clone().normalize(env.clone());
            let sup_kind = sup_norm.check_kind(env.clone())?;
            env.add_tyvar_super(outer_bound.var.clone(), *outer_bound.sup_ty.clone());
            env.add_tyvar_kind(outer_bound.var.clone(), sup_kind);

            let term_res = self.term.check(env.clone())?;
            let term_ty = term_res.ret_ty();
            let term_kind = term_ty.check_kind(env.clone())?;
            term_kind.check_equal(&outer_knd)?;

            let outer_subst = outer_bound.ty.subst_type(&outer_bound.var, &self.inner_ty);
            term_ty.check_subtype(&outer_subst, env.clone())?;
            let conc = TypingConclusion::new(env, self.clone(), self.outer_ty.clone());
            let deriv = TypingDerivation::pack_bound(conc, term_res);

            Ok(deriv.into())
        } else {
            Err(TypeMismatch::new(outer_norm.to_string(), "Existential Type".to_owned()).into())
        }
    }
}
