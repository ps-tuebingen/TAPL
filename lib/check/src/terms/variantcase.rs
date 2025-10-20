use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{EmptyCase, check_error::CheckError};
use errors::{TypeMismatch, UndefinedLabel};
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
        let bound_norm;

        if features.normalizing {
            let bound_norm_deriv = bound_ty.normalize(env.clone());
            bound_norm = bound_norm_deriv.ret_ty();
        } else {
            bound_norm = bound_ty;
        }

        if features.kinded {
            bound_norm.check_kind(env.clone())?.into_star()?;
        }

        let bound_var = bound_norm.into_variant()?;
        let mut rhs_tys = vec![];
        let mut rhs_knd = None;

        for pt in self.patterns.iter() {
            let var_ty = bound_var
                .variants
                .get(&pt.label)
                .cloned()
                .ok_or(UndefinedLabel::new(&pt.label))?;
            let var_norm;
            if features.normalizing {
                let var_norm_deriv = var_ty.normalize(env.clone());
                var_norm = var_norm_deriv.ret_ty();
                premises.push(var_norm_deriv);
            } else {
                var_norm = var_ty;
            }

            if features.kinded {
                var_norm.check_kind(env.clone())?;
            }

            let mut rhs_env = env.clone();
            rhs_env.add_var(pt.bound_var.clone(), var_norm);
            let rhs_res = pt.rhs.check(rhs_env.clone())?;
            let rhs_ty = rhs_res.ret_ty();
            let rhs_norm;
            if features.normalizing {
                let rhs_norm_deriv = rhs_ty.normalize(rhs_env);
                rhs_norm = rhs_norm_deriv.ret_ty();
                premises.push(rhs_norm_deriv);
            } else {
                rhs_norm = rhs_ty;
            }

            if features.kinded {
                let knd = rhs_norm.check_kind(env.clone())?;

                match rhs_knd {
                    None => {
                        rhs_knd = Some(knd);
                    }
                    Some(ref rhs) => {
                        rhs.check_equal(&knd)?;
                    }
                }
            }
            rhs_tys.push(rhs_norm)
        }

        if rhs_tys.is_empty() {
            return Err(EmptyCase.into());
        }

        let rhs_fst = rhs_tys.remove(0);
        if let Some(ty) = rhs_tys.iter().find(|ty| rhs_fst.check_equal(ty).is_err()) {
            return Err(TypeMismatch::new(ty.to_string(), rhs_fst.to_string()).into());
        }

        let conc = TypingConclusion::new(env, self.clone(), rhs_fst);
        let deriv = TypingDerivation::variantcase(conc, premises);
        Ok(deriv.into())
    }
}
