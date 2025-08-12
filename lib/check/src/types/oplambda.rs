use crate::{Kindcheck, Normalize, Subtypecheck};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    subst::SubstType,
    types::{OpLambda, Type, TypeGroup, TypeVariable},
};
impl<Ty> Subtypecheck<Ty> for OpLambda<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + SubstType<Ty, Target = Ty>,
    TypeVariable<Ty>: Into<Ty>,
{
    fn check_subtype(&self, sup: &Ty, mut env: Environment<Ty>) -> Result<(), CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_op = sup.clone().into_oplambda()?;
        sup_op.annot.check_equal(&self.annot)?;
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());

        self.body.check_subtype(
            &sup_op
                .body
                .subst_type(&sup_op.var, &(TypeVariable::new(&self.var).into())),
            env,
        )
    }
}

impl<Ty> Kindcheck<Ty> for OpLambda<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, mut env: Environment<Ty>) -> Result<Kind, CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        let body_kind = self.body.check_kind(env)?;
        Ok(Kind::Arrow(
            Box::new(self.annot.clone()),
            Box::new(body_kind),
        ))
    }
}

impl<Ty> Normalize<Ty> for OpLambda<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, env: Environment<Ty>) -> Ty {
        let body_norm = self.body.normalize(env);
        OpLambda {
            var: self.var,
            annot: self.annot,
            body: Box::new(body_norm),
        }
        .into()
    }
}
