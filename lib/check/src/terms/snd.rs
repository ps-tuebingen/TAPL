use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{env::Environment, language::Language, terms::Snd, types::TypeGroup};

impl<Lang> Typecheck for Snd<Lang>
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

        let ty_norm;
        if features.normalizing {
            let ty_norm_deriv = term_ty.normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            ty_norm = term_ty;
        }

        if features.kinded {
            premises.push(ty_norm.check_kind(env.clone())?);
        }

        let prod_ty = ty_norm.into_product()?;
        let conc = TypingConclusion::new(env, self.clone(), Rc::unwrap_or_clone(prod_ty.snd));
        let deriv = TypingDerivation::snd(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_cong(
            vec![Symbol::Term, SpecialChar::Dot.into(), Keyword::Snd.into()],
            Symbol::sub(Symbol::Type, 2),
            vec![
                Symbol::sub(Symbol::Type, 1),
                SpecialChar::Times.into(),
                Symbol::sub(Symbol::Type, 2),
            ],
            "T-Snd",
        )])
    }
}
