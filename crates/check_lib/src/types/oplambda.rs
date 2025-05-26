use crate::{Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::types::{OpLambda, TypeGroup};

impl<Ty> Subtypecheck<Ty> for OpLambda<Ty>
where
    Ty: LanguageType,
    TypeVariable<Ty>: Into<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_op = sup.clone().into_oplambda().map_err(to_subty_err)?;
        sup_op
            .annot
            .check_equal(&self.annot)
            .map_err(to_subty_err)?;
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
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        let body_kind = self.body.check_kind(env)?;
        Ok(Kind::Arrow(
            Box::new(self.annot.clone()),
            Box::new(body_kind),
        ))
    }
}
