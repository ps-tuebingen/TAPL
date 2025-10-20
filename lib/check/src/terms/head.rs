use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use std::rc::Rc;
use syntax::{env::Environment, language::Language, terms::Head, types::TypeGroup};

impl<Lang> Typecheck for Head<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty();
        premises.push(term_res);

        let term_norm;
        if features.normalizing {
            let term_norm_deriv = term_ty.normalize(env.clone());
            term_norm = term_norm_deriv.ret_ty();
            premises.push(term_norm_deriv);
        } else {
            term_norm = term_ty;
        }

        if features.kinded {
            term_norm.check_kind(env.clone())?.into_star()?;
        }

        let list_ty = term_norm.into_list()?;

        let conc =
            TypingConclusion::new(env.clone(), self.clone(), Rc::unwrap_or_clone(list_ty.ty));
        let deriv = TypingDerivation::head(conc, premises);
        Ok(deriv.into())
    }
}
