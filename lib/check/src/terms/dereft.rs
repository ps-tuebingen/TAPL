use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{env::Environment, language::Language, terms::Deref, types::TypeGroup};

impl<Lang> Typecheck for Deref<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty();
        premises.push(term_res);

        let term_norm;
        if features.normalizing {
            let term_norm_deriv = term_ty.normalize(env.clone());
            term_norm = term_norm_deriv.ret_ty();
            premises.push(term_norm_deriv);
        } else {
            term_norm = term_ty;
        }

        if features.kinded {
            let term_res = term_norm.check_kind(env.clone())?.into_kind()?;
            term_res.ret_kind().into_star()?;
            premises.push(term_res.into());
        }
        let ref_ty = term_norm.into_ref()?;

        let conc = TypingConclusion::new(env, self.clone(), Rc::unwrap_or_clone(ref_ty.ty));
        let deriv = TypingDerivation::deref(conc, premises);

        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_cong(
            vec![SpecialChar::Exclamation.into(), Symbol::Term],
            Symbol::Type,
            vec![Keyword::Ref.into(), Symbol::sqbrack(Symbol::Type)],
            "T-Deref",
        )])
    }
}
