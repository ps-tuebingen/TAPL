use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::Lambda,
    types::{Fun, Type},
};

impl<Lang> Typecheck for Lambda<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: Type + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Fun<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        if features.kinded {
            premises.push(self.annot.check_kind(env.clone())?);
        }

        env.add_var(self.var.clone(), self.annot.clone());
        let body_res = self.body.check(env.clone())?;
        let body_ty = body_res.ret_ty();
        premises.push(body_res);

        let body_norm;
        if features.normalizing {
            let body_norm_deriv = body_ty.normalize(env.clone());
            body_norm = body_norm_deriv.ret_ty();
            premises.push(body_norm_deriv);
        } else {
            body_norm = body_ty;
        }

        if features.kinded {
            premises.push(body_norm.check_kind(env.clone())?);
        }

        let conc = TypingConclusion::new(
            env.clone(),
            self.clone(),
            Fun::new(self.annot.clone(), body_norm).into(),
        );
        let deriv = TypingDerivation::lambda(conc, premises);

        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_lambda(false)])
    }
}
