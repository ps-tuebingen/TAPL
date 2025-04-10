use super::{check_subtype, to_check_err, Env};
use crate::{
    syntax::{LambdaSub, TyApp},
    traits::SubstTy,
    types::Type,
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for LambdaSub {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        env.add_tyvar(&self.var, &self.sup_ty);
        let body_ty = self.body.check(env)?;
        Ok(Type::Forall {
            var: self.var.clone(),
            sup_ty: Box::new(self.sup_ty.clone()),
            ty: Box::new(body_ty),
        })
    }
}
impl<'a> Typecheck<'a> for TyApp {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let t_ty = self.term.check(&mut env.clone())?;
        let (var, sup_ty, ty) = t_ty.as_forall().map_err(to_check_err)?;
        check_subtype(self.ty.clone(), sup_ty, env)?;
        Ok(ty.subst_ty(&var, self.ty.clone()))
    }
}
