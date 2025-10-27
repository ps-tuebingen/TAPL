use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use std::rc::Rc;
use syntax::{env::Environment, language::Language, terms::ListCase, types::TypeGroup};

impl<Lang> Typecheck for ListCase<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
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

        if features.kinded {
            let bound_res = bound_norm.check_kind(env.clone())?.into_kind()?;
            bound_res.ret_kind().into_star()?;
            premises.push(bound_res.into());
        }

        let bound_list = bound_norm.clone().into_list()?;

        let nil_res = self.nil_rhs.check(env.clone())?;
        let nil_ty = nil_res.ret_ty();
        premises.push(nil_res);

        let nil_norm;
        if features.normalizing {
            let nil_norm_deriv = nil_ty.normalize(env.clone());
            nil_norm = nil_norm_deriv.ret_ty();
            premises.push(nil_norm_deriv);
        } else {
            nil_norm = nil_ty;
        }

        env.add_var(self.cons_fst.clone(), Rc::unwrap_or_clone(bound_list.ty));
        env.add_var(self.cons_rst.clone(), bound_norm);
        let cons_res = self.cons_rhs.check(env.clone())?;
        let cons_ty = cons_res.ret_ty();
        premises.push(cons_res);

        let cons_norm;
        if features.normalizing {
            let cons_norm_deriv = cons_ty.normalize(env.clone());
            cons_norm = cons_norm_deriv.ret_ty();
            premises.push(cons_norm_deriv);
        } else {
            cons_norm = cons_ty;
        }

        if features.kinded {
            let nil_res = nil_norm.check_kind(env.clone())?.into_kind()?;
            let cons_res = cons_norm.check_kind(env.clone())?.into_kind()?;
            nil_res.ret_kind().check_equal(&cons_res.ret_kind())?;
            premises.push(nil_res.into());
            premises.push(cons_res.into());
        }

        nil_norm.check_equal(&cons_norm)?;
        let conc = TypingConclusion::new(env.clone(), self.clone(), cons_norm);
        let deriv = TypingDerivation::listcase(conc, premises);
        Ok(deriv.into())
    }
}
