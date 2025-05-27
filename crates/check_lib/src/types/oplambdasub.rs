use crate::{env::CheckEnvironment, to_subty_err, Kindcheck, Normalize, Subtypecheck};
use common::errors::Error;
use syntax::{
    kinds::Kind,
    subst::SubstType,
    types::{OpLambdaSub, Type, TypeGroup, TypeVariable},
};

impl<Ty> Subtypecheck<Ty> for OpLambdaSub<Ty>
where
    Ty: TypeGroup
        + Subtypecheck<Ty>
        + Normalize<Ty, Env = <Ty as Subtypecheck<Ty>>::Env>
        + SubstType<Ty, Target = Ty>,
    TypeVariable<Ty>: Into<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let sup_norm = sup.clone().normalize(env);
        let self_sup_norm = self.sup.clone().normalize(env);
        let sup_op = sup_norm.into_oplambdasub().map_err(to_subty_err)?;
        sup_op
            .sup
            .check_equal(&self_sup_norm)
            .map_err(to_subty_err)?;
        env.add_tyvar_super(self.var.clone(), self_sup_norm);

        self.body.check_subtype(
            &sup_op
                .body
                .subst_type(&sup_op.var, &(TypeVariable::new(&self.var).into())),
            env,
        )
    }
}

impl<Ty> Kindcheck<Ty> for OpLambdaSub<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        let sup_kind = self.sup.check_kind(env)?;
        env.add_tyvar_kind(self.var.clone(), sup_kind.clone());
        let body_kind = self.body.check_kind(env)?;
        Ok(Kind::Arrow(Box::new(sup_kind), Box::new(body_kind)))
    }
}

impl<Ty> Normalize<Ty> for OpLambdaSub<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        let body_norm = self.body.normalize(env);
        OpLambdaSub {
            var: self.var,
            sup: self.sup,
            body: Box::new(body_norm),
        }
        .into()
    }
}
