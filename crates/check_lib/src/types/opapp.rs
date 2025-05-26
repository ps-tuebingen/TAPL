use crate::{Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::types::{OpApp, TypeGroup};

impl<Ty> Subtypecheck<Ty> for OpApp<Ty>
where
    Ty: LanguageType,
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
    Ty: LanguageType,
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
