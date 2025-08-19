use crate::Typecheck;
use derivations::{DefinitionDerivation, Derivation};
use errors::check_error::CheckError;
use syntax::{
    definition::Definition,
    env::Environment,
    terms::Term,
    types::{Type, TypeGroup},
};

impl<T, Ty> Typecheck for Definition<T, Ty>
where
    T: Term + Typecheck<Term = T, Type = Ty>,
    Ty: Type + TypeGroup,
{
    type Term = T;
    type Type = Ty;
    fn check(&self, env: Environment<Ty>) -> Result<Derivation<T, Ty>, CheckError> {
        let body_res = self.body.check(env)?;
        let body_ty = body_res.ret_ty();
        self.annot.check_equal(&body_ty)?;
        Ok(DefinitionDerivation::new(&self.name, body_res.into_ty()?).into())
    }
}
