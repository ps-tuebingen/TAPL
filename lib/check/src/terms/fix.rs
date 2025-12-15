use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{KindMismatch, TypeMismatch, check_error::CheckError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{
    env::Environment,
    language::Language,
    terms::Fix,
    types::{Fun, TypeGroup},
};

impl<Lang> Typecheck for Fix<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Fun<Lang>: Into<Lang::Type>,
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

        let fun_ty = term_norm.clone().into_fun().ok_or(TypeMismatch::new(
            term_norm.to_string(),
            "Function Type".to_string(),
        ))?;
        if fun_ty.from != fun_ty.to {
            return Err(TypeMismatch::new(fun_ty.from.to_string(), fun_ty.to.to_string()).into());
        }

        let conc = TypingConclusion::new(env, self.clone(), Rc::unwrap_or_clone(fun_ty.to));
        let deriv = TypingDerivation::fix(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_cong(
            vec![Keyword::Fix.into(), Symbol::paren(Symbol::Term)],
            Symbol::Type,
            Symbol::arrow(Symbol::Type, Symbol::Type),
            "T-Fix",
        )])
    }
}
