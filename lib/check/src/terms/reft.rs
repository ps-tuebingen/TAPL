use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{env::Environment, language::Language, terms::Ref, types::Reference};

impl<Lang> Typecheck for Ref<Lang>
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

        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty();
        premises.push(term_res);

        let ty_norm;
        if features.normalizing {
            let ty_norm_deriv = term_ty.normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            ty_norm = term_ty;
        }

        if features.kinded {
            let ty_res = ty_norm.check_kind(env.clone())?.into_kind()?;
            ty_res.ret_kind().into_star()?;
            premises.push(ty_res.into());
        }

        let conc = TypingConclusion::new(env, self.clone(), Reference::new(ty_norm));
        let deriv = TypingDerivation::reft(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        let term = vec![Keyword::Ref.into(), Symbol::paren(Symbol::Term)];
        let ty_res = vec![Keyword::Ref.into(), Symbol::sqbrack(Symbol::Type)];
        HashSet::from([DerivationRule::check_cong(
            term,
            ty_res,
            Symbol::Type,
            "T-Ref",
        )])
    }
}
