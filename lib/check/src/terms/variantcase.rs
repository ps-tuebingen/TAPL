use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{EmptyCase, KindMismatch, TypeMismatch, UndefinedLabel, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{env::Environment, language::Language, terms::VariantCase, types::TypeGroup};

impl<Lang> Typecheck for VariantCase<Lang>
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
        let bound_norm = if features.normalizing() {
            let bound_norm_deriv = bound_ty.normalize(env.clone());
            bound_norm_deriv.ret_ty()
        } else {
            bound_ty
        };

        if features.kinded() {
            let bound_res = bound_norm.check_kind(env.clone())?.into_kind()?;
            let bound_knd = bound_res.ret_kind();
            bound_knd.clone().into_star().ok_or_else(|| {
                KindMismatch::new(bound_knd.to_string(), "Star Kind".to_string(), self.span)
            })?;
            premises.push(bound_res.into());
        }

        let bound_var = bound_norm.clone().into_variant().ok_or_else(|| {
            TypeMismatch::new(
                bound_norm.to_string(),
                "Variant Type".to_string(),
                self.span,
            )
        })?;
        let mut rhs_tys = vec![];
        let mut rhs_knd = None;

        for pt in &self.patterns {
            let var_ty = bound_var
                .variants
                .get(&pt.label)
                .cloned()
                .ok_or_else(|| UndefinedLabel::new(&pt.label, self.span))?;
            let var_norm;
            if features.normalizing() {
                let var_norm_deriv = var_ty.normalize(env.clone());
                var_norm = var_norm_deriv.ret_ty();
                premises.push(var_norm_deriv);
            } else {
                var_norm = var_ty;
            }

            if features.kinded() {
                premises.push(var_norm.check_kind(env.clone())?);
            }

            let mut rhs_env = env.clone();
            rhs_env.add_var(pt.bound_var.clone(), var_norm);
            let rhs_res = pt.rhs.check(rhs_env.clone())?;
            let rhs_ty = rhs_res.ret_ty();
            let rhs_norm;
            if features.normalizing() {
                let rhs_norm_deriv = rhs_ty.normalize(rhs_env);
                rhs_norm = rhs_norm_deriv.ret_ty();
                premises.push(rhs_norm_deriv);
            } else {
                rhs_norm = rhs_ty;
            }

            if features.kinded() {
                let rhs_res = rhs_norm.check_kind(env.clone())?.into_kind()?;
                let curr_rhs_knd = rhs_res.ret_kind();

                match rhs_knd {
                    None => {
                        rhs_knd = Some(curr_rhs_knd);
                    }
                    Some(ref rhs) if *rhs != curr_rhs_knd => {
                        return Err(KindMismatch::new(
                            rhs.to_string(),
                            curr_rhs_knd.to_string(),
                            self.span,
                        )
                        .into());
                    }
                    _ => (),
                }
                premises.push(rhs_res.into());
            }
            rhs_tys.push(rhs_norm);
        }

        if rhs_tys.is_empty() {
            return Err(EmptyCase::new(self.span).into());
        }

        let rhs_fst = rhs_tys.remove(0);
        if let Some(ty) = rhs_tys.iter().find(|ty| rhs_fst != **ty) {
            return Err(TypeMismatch::new(ty.to_string(), rhs_fst.to_string(), self.span).into());
        }

        let conc = TypingConclusion::new(env, self.clone(), rhs_fst);
        let deriv = TypingDerivation::variantcase(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_variantcase()])
    }
}
