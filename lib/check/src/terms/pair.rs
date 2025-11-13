use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
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
        let features = Lang::features();
        let mut premises = vec![];

        let fst_res = self.fst.check(env.clone())?;
        let fst_ty = fst_res.ret_ty();
        premises.push(fst_res);

        let fst_norm;
        if features.normalizing() {
            let fst_norm_deriv = fst_ty.normalize(env.clone());
            fst_norm = fst_norm_deriv.ret_ty();
            premises.push(fst_norm_deriv);
        } else {
            fst_norm = fst_ty;
        }

        let snd_res = self.snd.check(env.clone())?;
        let snd_ty = snd_res.ret_ty();
        premises.push(snd_res);

        let snd_norm;
        if features.normalizing() {
            let snd_norm_deriv = snd_ty.normalize(env.clone());
            snd_norm = snd_norm_deriv.ret_ty();
            premises.push(snd_norm_deriv);
        } else {
            snd_norm = snd_ty;
        }

        if features.kinded() {
            let fst_res = fst_norm.check_kind(env.clone())?.into_kind()?;
            let snd_res = snd_norm.check_kind(env.clone())?.into_kind()?;
            fst_res.ret_kind().check_equal(&snd_res.ret_kind())?;
            premises.push(fst_res.into());
            premises.push(snd_res.into());
        }

        let conc = TypingConclusion::new(env, self.clone(), Product::new(fst_norm, snd_norm));
        let deriv = TypingDerivation::pair(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_pair()])
    }
}
