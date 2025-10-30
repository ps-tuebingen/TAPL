use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{env::Environment, language::Language, terms::Raise, types::TypeGroup};

impl<Lang> Typecheck for Raise<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let err_res = self.exception.check(env.clone())?;
        let err_ty = err_res.ret_ty();
        premises.push(err_res);

        let ex_norm;
        let cont_norm;
        let err_norm;
        if features.normalizing {
            let ex_norm_deriv = self.exception_ty.clone().normalize(env.clone());
            let cont_norm_deriv = self.cont_ty.clone().normalize(env.clone());
            let err_norm_deriv = err_ty.normalize(env.clone());
            ex_norm = ex_norm_deriv.ret_ty();
            cont_norm = cont_norm_deriv.ret_ty();
            err_norm = err_norm_deriv.ret_ty();
            premises.push(ex_norm_deriv);
            premises.push(cont_norm_deriv);
            premises.push(err_norm_deriv);
        } else {
            ex_norm = self.exception_ty.clone();
            cont_norm = self.cont_ty.clone();
            err_norm = err_ty;
        }

        if features.kinded {
            let ex_res = ex_norm.check_kind(env.clone())?.into_kind()?;
            let cont_res = self.cont_ty.check_kind(env.clone())?;
            let err_res = err_norm.check_kind(env.clone())?.into_kind()?;
            ex_res.ret_kind().check_equal(&err_res.ret_kind())?;
            premises.push(ex_res.into());
            premises.push(cont_res);
            premises.push(err_res.into());
        }

        ex_norm.check_equal(&err_norm)?;
        let conc = TypingConclusion::new(env, self.clone(), cont_norm.clone());
        let deriv = TypingDerivation::raise(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_cong(
            vec![
                Keyword::Raise.into(),
                Symbol::sqbrack(Symbol::sub(Symbol::Type, "exn")),
                Symbol::paren(Symbol::Term),
            ],
            Symbol::Type,
            Symbol::sub(Symbol::Type, "exn"),
            "T-Raise",
        )])
    }
}
