use crate::{Kindcheck, Normalize};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::collections::HashSet;
use std::rc::Rc;
use syntax::{env::Environment, language::Language, types::Forall};

impl<Lang> Kindcheck for Forall<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(
        &self,
        mut env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        let ty_res = self.ty.check_kind(env)?.into_kind()?;
        Ok(KindingDerivation::forall(self.clone(), ty_res.ret_kind(), ty_res).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::kind_forall(false)])
    }
}

impl<Lang> Normalize for Forall<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, mut env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        let ty_norm = self.ty.clone().normalize(env);
        let self_norm = Forall {
            var: self.var.clone(),
            kind: self.kind.clone(),
            ty: Rc::new(ty_norm.ret_ty()),
        };
        NormalizingDerivation::cong(self, self_norm, vec![ty_norm]).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::norm_cong(|sym| {
            vec![
                SpecialChar::Forall.into(),
                Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Type),
                SpecialChar::Dot.into(),
                sym,
            ]
            .into()
        })])
    }
}
