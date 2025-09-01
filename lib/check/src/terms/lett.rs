use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Let};

impl<Lang> Typecheck for Let<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ret_ty().normalize(env.clone());
        bound_ty.check_kind(env.clone())?;

        env.add_var(self.var.clone(), bound_ty);
        let in_res = self.in_term.check(env.clone())?;
        let in_ty = in_res.ret_ty().normalize(env.clone());
        in_ty.check_kind(env.clone())?;

        let conc = TypingConclusion::new(env.clone(), self.clone(), in_ty);
        let deriv = TypingDerivation::lett(conc, bound_res, in_res);
        Ok(deriv.into())
    }
}
