use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{KindMismatch, TypeMismatch, check_error::CheckError};
use grammar::DerivationRule;
use std::{collections::HashSet, rc::Rc};
use syntax::{env::Environment, language::Language, terms::SomeCase, types::TypeGroup};

impl<Lang> Typecheck for SomeCase<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ret_ty();
        premises.push(bound_res);

        let bound_norm;
        if features.normalizing() {
            let bound_norm_deriv = bound_ty.normalize(env.clone());
            bound_norm = bound_norm_deriv.ret_ty();
            premises.push(bound_norm_deriv);
        } else {
            bound_norm = bound_ty;
        }

        if features.kinded() {
            let bound_res = bound_norm.check_kind(env.clone())?.into_kind()?;
            let bound_knd = bound_res.ret_kind();
            bound_knd.clone().into_star().ok_or(KindMismatch::new(
                bound_knd.to_string(),
                "Star Kind".to_string(),
            ))?;
            premises.push(bound_res.into());
        }

        let option = bound_norm.clone().into_optional().ok_or(TypeMismatch::new(
            bound_norm.to_string(),
            "Option Type".to_string(),
        ))?;
        let mut some_env = env.clone();
        some_env.add_var(self.some_var.clone(), Rc::unwrap_or_clone(option.ty));
        let some_res = self.some_term.check(some_env.clone())?;
        let some_ty = some_res.ret_ty();
        premises.push(some_res);

        let some_norm;
        if features.normalizing() {
            let some_norm_deriv = some_ty.normalize(some_env.clone());
            some_norm = some_norm_deriv.ret_ty();
            premises.push(some_norm_deriv);
        } else {
            some_norm = some_ty;
        }

        let none_res = self.none_term.check(env.clone())?;
        let none_ty = none_res.ret_ty();
        premises.push(none_res);

        let none_norm;
        if features.normalizing() {
            let none_norm_res = none_ty.normalize(env.clone());
            none_norm = none_norm_res.ret_ty();
            premises.push(none_norm_res);
        } else {
            none_norm = none_ty;
        }

        if features.kinded() {
            let some_res = some_norm.check_kind(some_env)?.into_kind()?;
            let none_res = none_norm.check_kind(env.clone())?.into_kind()?;
            let some_knd = some_res.ret_kind();
            let none_knd = none_res.ret_kind();
            if some_knd != none_knd {
                return Err(KindMismatch::new(some_knd.to_string(), none_knd.to_string()).into());
            }
            premises.push(some_res.into());
            premises.push(none_res.into());
        }

        if some_norm != none_norm {
            return Err(TypeMismatch::new(some_norm.to_string(), none_norm.to_string()).into());
        }

        let conc = TypingConclusion::new(env, self.clone(), some_norm);
        let deriv = TypingDerivation::somecase(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_somecase()])
    }
}
