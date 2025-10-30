use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::Nil,
    types::{List, Type},
};

impl<Lang> Typecheck for Nil<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: Type + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    List<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let ty_norm;
        if features.normalizing {
            let ty_norm_deriv = self.ty.clone().normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            ty_norm = self.ty.clone();
        }

        if features.kinded {
            let ty_res = ty_norm.check_kind(env.clone())?.into_kind()?;
            ty_res.ret_kind().into_star()?;
            premises.push(ty_res.into());
        }

        let conc =
            TypingConclusion::new(env.clone(), self.clone(), List::new(ty_norm.clone()).into());
        let deriv = TypingDerivation::nil(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_const(
            vec![Keyword::Nil.into(), Symbol::sqbrack(Symbol::Type)],
            vec![Keyword::List.into(), Symbol::sqbrack(Symbol::Type)],
            "T-Nil",
        )])
    }
}
