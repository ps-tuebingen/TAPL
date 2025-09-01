use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::TyLambda, types::Forall};

impl<Lang> Typecheck for TyLambda<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Forall<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        let term_knd = term_ty.check_kind(env.clone())?;
        self.annot.check_equal(&term_knd)?;
        let ty = Forall::new(&self.var, self.annot.clone(), term_ty);
        let conc = TypingConclusion::new(env, self.clone(), ty);
        let deriv = TypingDerivation::tylambda(conc, term_res);
        Ok(deriv.into())
    }
}
