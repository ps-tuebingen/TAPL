use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::If, types::TypeGroup};

impl<Lang> Typecheck for If<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let if_res = self.if_cond.check(env.clone())?;
        let if_ty = if_res.ret_ty().normalize(env.clone());
        if_ty.check_kind(env.clone())?.into_star()?;
        if_ty.into_bool()?;

        let then_res = self.then_term.check(env.clone())?;
        let then_ty = then_res.ret_ty().normalize(env.clone());
        let then_kind = then_ty.check_kind(env.clone())?;

        let else_res = self.else_term.check(env.clone())?;
        let else_ty = else_res.ret_ty().normalize(env.clone());
        let else_kind = else_ty.check_kind(env.clone())?;

        then_kind.check_equal(&else_kind)?;
        then_ty.check_equal(&else_ty)?;

        let conc = TypingConclusion::new(env.clone(), self.clone(), then_ty);
        let deriv = TypingDerivation::ift(conc, if_res, then_res, else_res);
        Ok(deriv.into())
    }
}
