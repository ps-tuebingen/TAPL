use crate::{to_kind_err, to_subty_err, Kindcheck, Normalize, Subtypecheck};
use common::errors::{Error, ErrorKind};
use syntax::{
    kinds::Kind,
    subst::SubstType,
    types::{OpApp, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for OpApp<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
    Self: Into<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }
        let sup_op = sup.clone().into_opapp().map_err(to_subty_err)?;
        self.fun.check_subtype(&sup_op.fun, &mut env.clone())?;
        self.arg.check_subtype(&sup_op.arg, env)
    }
}

impl<Ty> Kindcheck<Ty> for OpApp<Ty>
where
    Ty: TypeGroup + Kindcheck<Ty>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        let fun_kind = self.fun.check_kind(&mut env.clone())?;
        let (fun_from, fun_to) = fun_kind.into_arrow().map_err(to_kind_err)?;
        let arg_kind = self.arg.check_kind(env)?;
        if fun_from == arg_kind {
            Ok(fun_to)
        } else {
            Err(to_kind_err(ErrorKind::KindMismatch {
                found: arg_kind.to_string(),
                expected: fun_from.to_string(),
            }))
        }
    }
}

impl<Ty> Normalize<Ty> for OpApp<Ty>
where
    Ty: TypeGroup + Normalize<Ty> + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        let fun_norm = self.fun.normalize(env);
        if let Ok(oplam) = fun_norm.clone().into_oplambda() {
            oplam.body.subst_type(&oplam.var, &self.arg).normalize(env)
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
