use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Pair, types::Product};

impl<Lang> Typecheck for Pair<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Product<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let fst_res = self.fst.check(env.clone())?;
        let fst_ty = fst_res.ret_ty().normalize(env.clone());

        let snd_res = self.snd.check(env.clone())?;
        let snd_ty = snd_res.ret_ty().normalize(env.clone());

        let fst_knd = fst_ty.check_kind(env.clone())?;
        let snd_knd = snd_ty.check_kind(env.clone())?;
        fst_knd.check_equal(&snd_knd)?;

        let conc = TypingConclusion::new(env, self.clone(), Product::new(fst_ty, snd_ty));
        let deriv = TypingDerivation::pair(conc, fst_res, snd_res);
        Ok(deriv.into())
    }
}
