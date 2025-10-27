use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Something, types::Optional};

impl<Lang> Typecheck for Something<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Optional<Lang>: Into<Lang::Type>,
    <Lang as Language>::Type: Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
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
        }

        if features.kinded {
            let ty_res = ty_norm.check_kind(env.clone())?.into_kind()?;
            ty_res.ret_kind().into_star()?;
            premises.push(ty_res.into());
        }
        let conc = TypingConclusion::new(env, self.clone(), Optional::new(ty_norm.clone()));
        let deriv = TypingDerivation::something(conc, premises);
        Ok(deriv.into())
    }
}
