use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::TypeMismatch;
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::{collections::HashSet, rc::Rc};
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
        let features = Lang::features();
        let mut premises = vec![];

        let outer_norm;
        if features.normalizing {
            let outer_norm_deriv = self.outer_ty.clone().normalize(env.clone());
            outer_norm = outer_norm_deriv.ret_ty();
            premises.push(outer_norm_deriv);
        } else {
            outer_norm = self.outer_ty.clone();
        }

        if let Ok(outer_exists) = outer_norm.clone().into_exists() {
            env.add_tyvar_kind(outer_exists.var.clone(), outer_exists.kind.clone());
            let term_res = self.term.check(env.clone())?;
            let term_ty = term_res.ret_ty();
            premises.push(term_res);

            let ty_norm;
            if features.normalizing {
                let ty_norm_deriv = term_ty.normalize(env.clone());
                ty_norm = ty_norm_deriv.ret_ty();
                premises.push(ty_norm_deriv);
            } else {
                ty_norm = term_ty;
            }

            if features.kinded {
                let term_res = ty_norm.check_kind(env.clone())?.into_kind()?;
                let outer_res = outer_exists.ty.check_kind(env.clone())?.into_kind()?;
                let inner_res = self.inner_ty.check_kind(env.clone())?.into_kind()?;
                term_res.ret_kind().check_equal(&outer_res.ret_kind())?;
                inner_res.ret_kind().check_equal(&outer_exists.kind)?;
                premises.push(term_res.into());
                premises.push(outer_res.into());
                premises.push(inner_res.into());
            };

            let outer_subst = Rc::unwrap_or_clone(
                outer_exists
                    .ty
                    .subst_type(&outer_exists.var, &self.inner_ty),
            );
            let outer_subst_norm;
            if features.normalizing {
                let outer_subst_norm_deriv = outer_subst.normalize(env.clone());
                outer_subst_norm = outer_subst_norm_deriv.ret_ty();
                premises.push(outer_subst_norm_deriv);
            } else {
                outer_subst_norm = outer_subst;
            }

            outer_subst_norm.check_equal(&ty_norm)?;

            let conc = TypingConclusion::new(env, self.clone(), self.outer_ty.clone());
            let deriv = TypingDerivation::pack(conc, premises);
            Ok(deriv.into())
        } else if let Ok(outer_bound) = outer_norm.clone().into_exists_bounded() {
            let sup_norm;
            if features.normalizing {
                let sup_norm_deriv = outer_bound.sup_ty.clone().normalize(env.clone());
                sup_norm = sup_norm_deriv.ret_ty();
                premises.push(sup_norm_deriv);
            } else {
                sup_norm = Rc::unwrap_or_clone(outer_bound.sup_ty.clone());
            }

            if features.kinded {
                let sup_res = sup_norm.check_kind(env.clone())?.into_kind()?;
                env.add_tyvar_kind(outer_bound.var.clone(), sup_res.ret_kind());
                premises.push(sup_res.into());
            }

            env.add_tyvar_super(
                outer_bound.var.clone(),
                Rc::unwrap_or_clone(outer_bound.sup_ty.clone()),
            );

            let term_res = self.term.check(env.clone())?;
            let term_ty = term_res.ret_ty();
            premises.push(term_res);

            if features.kinded {
                let term_res = term_ty.check_kind(env.clone())?.into_kind()?;
                let outer_res = outer_bound.ty.check_kind(env.clone())?.into_kind()?;
                term_res.ret_kind().check_equal(&outer_res.ret_kind())?;
                premises.push(term_res.into());
                premises.push(outer_res.into());
            }

            let outer_subst = outer_bound.ty.subst_type(&outer_bound.var, &self.inner_ty);

            if features.subtyped {
                let sup_deriv = term_ty.check_subtype(&outer_subst, env.clone())?;
                premises.push(sup_deriv);
            }
            let conc = TypingConclusion::new(env, self.clone(), self.outer_ty.clone());
            let deriv = TypingDerivation::pack_bound(conc, premises);

            Ok(deriv.into())
        } else {
            Err(TypeMismatch::new(outer_norm.to_string(), "Existential Type".to_owned()).into())
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let features = Lang::features();
        HashSet::from([DerivationRule::check_pack(features.subtyped)])
    }
}
