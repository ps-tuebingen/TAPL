use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation, SubtypeDerivation};
use errors::{NameMismatch, check_error::CheckError};
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::collections::HashSet;
use std::rc::Rc;
use syntax::{
    env::Environment,
    language::Language,
    types::{ForallBounded, Top, TypeGroup},
};

impl<Lang> Subtypecheck for ForallBounded<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Top<Lang>: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang> + TypeGroup<Lang = Lang> + Subtypecheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, vec![]).into());
        }

        let other_forall = sup.clone().into_forall_bounded()?;

        if self.var != other_forall.var {
            return Err(NameMismatch::new(&other_forall.var, &self.var).into());
        }

        let other_sup_norm;
        let self_sup_norm;
        let self_norm;
        if features.normalizing() {
            let other_sup_norm_deriv = other_forall.sup_ty.normalize(env.clone());
            other_sup_norm = other_sup_norm_deriv.ret_ty();
            let self_sup_norm_deriv = self.sup_ty.clone().normalize(env.clone());
            self_sup_norm = self_sup_norm_deriv.ret_ty();
            let self_norm_deriv = self.ty.clone().normalize(env.clone());
            self_norm = self_norm_deriv.ret_ty();
            premises.push(other_sup_norm_deriv);
            premises.push(self_sup_norm_deriv);
            premises.push(self_norm_deriv);
        } else {
            other_sup_norm = Rc::unwrap_or_clone(other_forall.sup_ty);
            self_sup_norm = Rc::unwrap_or_clone(self.sup_ty.clone());
            self_norm = Rc::unwrap_or_clone(self.ty.clone());
        }

        let sup_res = other_sup_norm.check_subtype(&self_sup_norm, env.clone())?;
        let inner_res = self_norm.check_subtype(&(*other_forall.ty), env.clone())?;
        premises.push(sup_res);
        premises.push(inner_res);

        Ok(SubtypeDerivation::forall_bounded(env, self.clone(), sup.clone(), premises).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_forall(true)])
    }
}

impl<Lang> Kindcheck for ForallBounded<Lang>
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
        let sup_res = self.sup_ty.check_kind(env.clone())?.into_kind()?;
        env.add_tyvar_kind(self.var.clone(), sup_res.ret_kind());
        let inner_res = self.ty.check_kind(env)?.into_kind()?;
        Ok(
            KindingDerivation::forall_bound(self.clone(), inner_res.ret_kind(), sup_res, inner_res)
                .into(),
        )
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::kind_forall(true)])
    }
}

impl<Lang> Normalize for ForallBounded<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, mut env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        env.add_tyvar_super(self.var.clone(), Rc::unwrap_or_clone(self.ty.clone()));
        let ty_norm = self.ty.clone().normalize(env);
        let self_norm = Self {
            var: self.var.clone(),
            sup_ty: self.sup_ty.clone(),
            ty: Rc::new(ty_norm.ret_ty()),
        };
        NormalizingDerivation::cong(self, self_norm, vec![ty_norm]).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::norm_cong(|sym| {
            vec![
                SpecialChar::Forall.into(),
                Symbol::less_colon_sep(Symbol::Typevariable, Symbol::Type),
                SpecialChar::Dot.into(),
                sym,
            ]
            .into()
        })])
    }
}
