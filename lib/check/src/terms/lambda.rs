use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
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
        self.annot.check_kind(env.clone())?;
        env.add_var(self.var.clone(), self.annot.clone());
        let body_res = self.body.check(env.clone())?;
        let body_ty = body_res.ret_ty().normalize(env.clone());
        body_ty.check_kind(env.clone())?;

        let conc = TypingConclusion::new(
            env.clone(),
            self.clone(),
            Fun::new(self.annot.clone(), body_ty).into(),
        );
        let deriv = TypingDerivation::lambda(conc, body_res);

        Ok(deriv.into())
    }
}
