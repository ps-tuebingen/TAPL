use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Loc, types::Reference};

impl<Lang> Typecheck for Loc<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Reference<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let loc_ty = env.get_loc(&self.loc)?;
        let loc_norm;
        if features.normalizing {
            let loc_norm_deriv = loc_ty.normalize(env.clone());
            loc_norm = loc_norm_deriv.ret_ty();
            premises.push(loc_norm_deriv);
        } else {
            loc_norm = loc_ty;
        }

        if features.kinded {
            loc_norm.check_kind(env.clone())?.into_star()?;
        }

        let conc = TypingConclusion::new(env, self.clone(), Reference::new(loc_norm));
        let deriv = TypingDerivation::loc(conc, premises);
        Ok(deriv.into())
    }
}
