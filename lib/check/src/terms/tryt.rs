use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Try, types::TypeGroup};

impl<Lang> Typecheck for Try<Lang>
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
        };

        let handler_res = self.handler.check(env.clone())?;
        let handler_ty = handler_res.ret_ty();
        premises.push(handler_res);

        let handler_norm;
        if features.normalizing {
            let handler_norm_deriv = handler_ty.normalize(env.clone());
            handler_norm = handler_norm_deriv.ret_ty();
            premises.push(handler_norm_deriv);
        } else {
            handler_norm = handler_ty;
        }

        ty_norm.check_equal(&handler_norm)?;

        if features.kinded {
            let term_res = ty_norm.check_kind(env.clone())?.into_kind()?;
            let handler_res = handler_norm.check_kind(env.clone())?.into_kind()?;
            term_res.ret_kind().check_equal(&handler_res.ret_kind())?;
            premises.push(term_res.into());
            premises.push(handler_res.into());
        }

        let conc = TypingConclusion::new(env, self.clone(), ty_norm);
        let deriv = TypingDerivation::tryt(conc, premises);
        Ok(deriv.into())
    }
}
