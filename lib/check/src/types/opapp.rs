use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, NormalizingDerivation, SubtypeDerivation};
use errors::KindMismatch;
use errors::check_error::CheckError;
use std::rc::Rc;
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
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
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, vec![]).into());
        }
        let sup_op = sup.clone().into_opapp()?;
        let fun_res = self.fun.check_subtype(&sup_op.fun, env.clone())?;
        self.arg.check_equal(&sup_op.arg)?;
        Ok(SubtypeDerivation::op_app(
            env,
            Rc::unwrap_or_clone(self.fun.clone()),
            Rc::unwrap_or_clone(sup_op.fun),
            Rc::unwrap_or_clone(self.arg.clone()),
            fun_res,
        )
        .into())
    }
}

impl<Lang> Kindcheck for OpApp<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        let fun_kind = self.fun.check_kind(env.clone())?;
        let (fun_from, fun_to) = fun_kind.into_arrow()?;
        let arg_kind = self.arg.check_kind(env)?;
        if fun_from == arg_kind {
            Ok(fun_to)
        } else {
            Err(KindMismatch::new(arg_kind.to_string(), fun_from.to_string()).into())
        }
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
        if let Ok(oplam) = fun_norm.clone().into_oplambda() {
            let oplam_deriv = oplam
                .body
                .subst_type(&oplam.var, &self.arg)
                .normalize(env.clone());
            let body_norm = oplam_deriv.ret_ty();
            premises.push(oplam_deriv);
            NormalizingDerivation::opapp(self, body_norm, premises).into()
        } else if let Ok(oplam) = fun_norm.clone().into_oplambdasub() {
            let oplam_deriv = oplam.body.subst_type(&oplam.var, &self.arg).normalize(env);
            let body_norm = oplam_deriv.ret_ty();
            premises.push(oplam_deriv);
            NormalizingDerivation::opapp(self, body_norm, premises).into()
        } else {
            let arg_deriv = self.arg.clone().normalize(env);
            let body_norm = OpApp {
                fun: Rc::new(fun_norm),
                arg: Rc::new(arg_deriv.ret_ty()),
            };
            premises.push(arg_deriv);
            NormalizingDerivation::cong(self, body_norm, premises).into()
        }
    }
}
