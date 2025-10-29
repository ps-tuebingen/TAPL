use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{NameMismatch, TypeMismatch, check_error::CheckError};
use grammar::DerivationRule;
use std::{collections::HashSet, rc::Rc};
use syntax::{env::Environment, language::Language, terms::Unpack, types::TypeGroup};

impl<Lang> Typecheck for Unpack<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ret_ty();
        premises.push(bound_res);

        let bound_norm;
        if features.normalizing {
            let bound_norm_deriv = bound_ty.normalize(env.clone());
            bound_norm = bound_norm_deriv.ret_ty();
            premises.push(bound_norm_deriv);
        } else {
            bound_norm = bound_ty;
        }

        if let Ok(bound_exists) = bound_norm.clone().into_exists() {
            if self.ty_name != bound_exists.var {
                return Err(NameMismatch::new(&bound_exists.var, &self.ty_name).into());
            }
            env.add_tyvar_kind(bound_exists.var, bound_exists.kind);
            env.add_var(self.term_name.clone(), Rc::unwrap_or_clone(bound_exists.ty));

            let in_res = self.in_term.check(env.clone())?;
            let in_ty = in_res.ret_ty();
            premises.push(in_res);

            let in_norm;
            if features.normalizing {
                let in_norm_deriv = in_ty.normalize(env.clone());
                in_norm = in_norm_deriv.ret_ty();
                premises.push(in_norm_deriv);
            } else {
                in_norm = in_ty;
            }

            let conc = TypingConclusion::new(env, self.clone(), in_norm);
            let deriv = TypingDerivation::unpack(conc, premises);
            Ok(deriv.into())
        } else if let Ok(bound_bound) = bound_norm.clone().into_exists_bounded() {
            if self.ty_name != bound_bound.var {
                return Err(NameMismatch::new(&bound_bound.var, &self.ty_name).into());
            }

            if features.kinded {
                let sup_res = bound_bound.sup_ty.check_kind(env.clone())?.into_kind()?;
                env.add_tyvar_kind(self.ty_name.clone(), sup_res.ret_kind());
                premises.push(sup_res.into());
            }

            env.add_tyvar_super(bound_bound.var, Rc::unwrap_or_clone(bound_bound.sup_ty));
            env.add_var(self.term_name.clone(), Rc::unwrap_or_clone(bound_bound.ty));

            let inner_res = self.in_term.check(env.clone())?;
            let inner_ty = inner_res.ret_ty();
            premises.push(inner_res);

            let inner_norm;
            if features.normalizing {
                let inner_norm_deriv = inner_ty.normalize(env.clone());
                inner_norm = inner_norm_deriv.ret_ty();
                premises.push(inner_norm_deriv);
            } else {
                inner_norm = inner_ty;
            }

            let conc = TypingConclusion::new(env.clone(), self.clone(), inner_norm);
            let deriv = TypingDerivation::unpack_bounded(conc, premises);
            Ok(deriv.into())
        } else {
            Err(TypeMismatch::new(bound_norm.to_string(), "Existential Type".to_owned()).into())
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let features = Lang::features();
        HashSet::from([DerivationRule::check_unpack(features.subtyped)])
    }
}
