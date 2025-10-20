use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    terms::Nothing,
    types::{Optional, Type},
};

impl<Lang> Typecheck for Nothing<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: Type + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Optional<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let ty_norm;
        if features.normalizing {
            let ty_norm_deriv = self.ty.clone().normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            ty_norm = self.ty.clone();
        }

        if features.kinded {
            ty_norm.check_kind(env.clone())?.into_star()?;
        }

        let conc = TypingConclusion::new(
            env.clone(),
            self.clone(),
            Optional::new(ty_norm.clone()).into(),
        );
        let deriv = TypingDerivation::nothing(conc, premises);
        Ok(deriv.into())
    }
}
