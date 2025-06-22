use crate::{Kindcheck, Normalize, Subtypecheck, errors::CheckError};
use syntax::{
    env::Environment,
    kinds::Kind,
    subst::SubstType,
    types::{OpLambdaSub, Type, TypeGroup, TypeVariable},
};
impl<Ty> Subtypecheck<Ty> for OpLambdaSub<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + Normalize<Ty> + SubstType<Ty, Target = Ty>,
    TypeVariable<Ty>: Into<Ty>,
{
    fn check_subtype(&self, sup: &Ty, mut env: Environment<Ty>) -> Result<(), CheckError> {
        let sup_norm = sup.clone().normalize(env.clone());
        let self_sup_norm = self.sup.clone().normalize(env.clone());
        let sup_op = sup_norm.into_oplambdasub()?;
        sup_op.sup.check_equal(&self_sup_norm)?;
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
    fn check_kind(&self, mut env: Environment<Ty>) -> Result<Kind, CheckError> {
        let sup_kind = self.sup.check_kind(env.clone())?;
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
    fn normalize(self, env: Environment<Ty>) -> Ty {
        let body_norm = self.body.normalize(env);
        OpLambdaSub {
            var: self.var,
            sup: self.sup,
            body: Box::new(body_norm),
        }
        .into()
    }
}
