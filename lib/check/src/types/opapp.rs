use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation, SubtypeDerivation};
use errors::{KindMismatch, TypeMismatch, check_error::CheckError};
use grammar::{DerivationRule, symbols::Symbol};
use std::{collections::HashSet, rc::Rc};
use syntax::{
    env::Environment,
    language::Language,
    span::Spanned,
    subst::SubstType,
    types::{OpApp, Top, TypeGroup},
};

impl<Lang> Subtypecheck for OpApp<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Top<Lang>: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang> + TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Some(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(
                env,
                self.clone(),
                top.kind,
                self.span,
                Vec::new(),
            )
            .into());
        }
        let sup_op = sup.clone().into_opapp().ok_or_else(|| {
            TypeMismatch::new(
                sup.to_string(),
                "Operator Application".to_string(),
                self.span,
            )
        })?;
        let fun_res = self.fun.check_subtype(&sup_op.fun, env.clone())?;
        if self.arg != sup_op.arg {
            return Err(
                TypeMismatch::new(self.arg.to_string(), sup_op.arg.to_string(), self.span).into(),
            );
        }
        Ok(SubtypeDerivation::op_app(
            env,
            Rc::unwrap_or_clone(self.fun.clone()),
            Rc::unwrap_or_clone(sup_op.fun),
            Rc::unwrap_or_clone(self.arg.clone()),
            self.fun.span(),
            self.arg.span(),
            fun_res,
        )
        .into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_cong(|sym| {
            vec![sym, Symbol::brack(Symbol::Type)].into()
        })])
    }
}

impl<Lang> Kindcheck for OpApp<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        let fun_res = self.fun.check_kind(env.clone())?.into_kind()?;
        let fun_kind = fun_res.ret_kind();
        let (fun_from, fun_to) = fun_kind.clone().into_arrow().ok_or_else(|| {
            KindMismatch::new(fun_kind.to_string(), "Arrow Kind".to_string(), self.span)
        })?;
        let arg_res = self.arg.check_kind(env)?.into_kind()?;
        let arg_kind = arg_res.ret_kind();
        if fun_from != arg_kind {
            return Err(
                KindMismatch::new(arg_kind.to_string(), fun_from.to_string(), self.span).into(),
            );
        }
        Ok(KindingDerivation::op_app(self.clone(), fun_to, fun_res, arg_res).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::kind_op_app()])
    }
}

impl<Lang> Normalize for OpApp<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang> + TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        let mut premises = vec![];
        let fun_norm_deriv = self.fun.clone().normalize(env.clone());
        let fun_norm = fun_norm_deriv.ret_ty();
        premises.push(fun_norm_deriv);
        if let Some(oplam) = fun_norm.clone().into_oplambda() {
            let oplam_deriv = oplam.body.subst_type(&oplam.var, &self.arg).normalize(env);
            let body_norm = oplam_deriv.ret_ty();
            premises.push(oplam_deriv);
            NormalizingDerivation::opapp(self, body_norm, premises).into()
        } else if let Some(oplam) = fun_norm.clone().into_oplambdasub() {
            let oplam_deriv = oplam.body.subst_type(&oplam.var, &self.arg).normalize(env);
            let body_norm = oplam_deriv.ret_ty();
            premises.push(oplam_deriv);
            NormalizingDerivation::opapp(self, body_norm, premises).into()
        } else {
            let arg_deriv = self.arg.clone().normalize(env);
            let body_norm = Self {
                fun: Rc::new(fun_norm),
                arg: Rc::new(arg_deriv.ret_ty()),
                span: self.span,
            };
            premises.push(arg_deriv);
            NormalizingDerivation::cong(self, body_norm, premises).into()
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let features = Lang::features();
        HashSet::from([
            DerivationRule::norm_cong(|sym| vec![sym, Symbol::brack(Symbol::Type)].into()),
            DerivationRule::norm_cong(|sym| vec![Symbol::Type, Symbol::brack(sym)].into()),
            DerivationRule::norm_ap(features.subtyped()),
        ])
    }
}
