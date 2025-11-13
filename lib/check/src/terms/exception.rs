use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{env::Environment, language::Language, terms::Exception, types::Type};

impl<Lang> Typecheck for Exception<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: Type + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let ty_norm;
        if features.normalizing() {
            let ty_norm_deriv = self.ty.clone().normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            ty_norm = self.ty.clone();
        }

        if features.kinded() {
            premises.push(ty_norm.check_kind(env.clone())?);
        }

        let conc = TypingConclusion::new(env, self.clone(), ty_norm);
        let deriv = TypingDerivation::exception(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_const(
            vec![
                Keyword::Err.into(),
                Symbol::sqbrack(Symbol::Type),
                SpecialChar::SqBrackC.into(),
            ],
            Symbol::Type,
            "T-Error",
        )])
    }
}
