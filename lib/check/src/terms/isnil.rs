use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{KindMismatch, TypeMismatch, check_error::CheckError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::IsNil,
    types::{Bool, List, TypeGroup},
};

impl<Lang> Typecheck for IsNil<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    List<Lang>: Into<Lang::Type>,
    Bool<Lang>: Into<Lang::Type>,
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
        if features.normalizing() {
            let term_norm_deriv = term_ty.normalize(env.clone());
            term_norm = term_norm_deriv.ret_ty();
            premises.push(term_norm_deriv);
        } else {
            term_norm = term_ty;
        }

        if features.kinded() {
            let term_res = term_norm.check_kind(env.clone())?.into_kind()?;
            let term_knd = term_res.ret_kind();
            term_knd.clone().into_star().ok_or(KindMismatch::new(
                term_knd.to_string(),
                "Star Kind".to_string(),
            ))?;
            premises.push(term_res.into());
        }
        term_norm.clone().into_list().ok_or(TypeMismatch::new(
            term_norm.to_string(),
            "List Type".to_string(),
        ))?;

        let conc = TypingConclusion::new(env, self.clone(), Bool::new());
        let deriv = TypingDerivation::isnil(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_cong(
            vec![
                Keyword::IsNil.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(Symbol::Term),
            ],
            Keyword::Bool,
            vec![Keyword::List.into(), Symbol::sqbrack(Symbol::Type)],
            "T-IsNil",
        )])
    }
}
