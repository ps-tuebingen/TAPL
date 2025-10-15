use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::KindMismatch;
use errors::check_error::CheckError;
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
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }
        let sup_op = sup.clone().into_opapp()?;
        let fun_res = self.fun.check_subtype(&sup_op.fun, env.clone())?;
        self.arg.check_equal(&sup_op.arg)?;
        Ok(SubtypeDerivation::op_app(
            env,
            *self.fun.clone(),
            *sup_op.fun,
            *self.arg.clone(),
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
    fn normalize(self, env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        let fun_norm = self.fun.normalize(env.clone());
        if let Ok(oplam) = fun_norm.clone().into_oplambda() {
            oplam
                .body
                .subst_type(&oplam.var, &self.arg)
                .normalize(env.clone())
        } else if let Ok(oplam) = fun_norm.clone().into_oplambdasub() {
            oplam.body.subst_type(&oplam.var, &self.arg).normalize(env)
        } else {
            OpApp {
                fun: Box::new(fun_norm),
                arg: Box::new(self.arg.normalize(env)),
            }
            .into()
        }
    }
}
