use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    terms::IsZero,
    types::{Bool, TypeGroup},
};

impl<Lang> Typecheck for IsZero<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Bool<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let inner_res = self.term.check(env.clone())?;
        let inner_ty = inner_res.ret_ty().normalize(env.clone());
        inner_ty.check_kind(env.clone())?.into_star()?;
        inner_ty.into_nat()?;

        let conc = TypingConclusion::new(env, self.clone(), Bool::new());
        let deriv = TypingDerivation::iszero(conc, inner_res);
        Ok(deriv.into())
    }
}
