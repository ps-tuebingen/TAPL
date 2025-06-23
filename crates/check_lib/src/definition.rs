use crate::{errors::CheckError, Typecheck};
use derivation::{DefinitionDerivation, TypingDerivation};
use syntax::{
    definition::Definition,
    env::Environment,
    terms::Term,
    types::{Type, TypeGroup},
};

impl<T, Ty> Typecheck for Definition<T, Ty>
where
    T: Term + Typecheck<Term = T, Type = Ty, Deriv = TypingDerivation<T, Ty>>,
    Ty: Type + TypeGroup,
{
    type Term = T;
    type Type = Ty;
    type Deriv = DefinitionDerivation<T, Ty>;
    fn check(&self, env: Environment<Ty>) -> Result<Self::Deriv, CheckError> {
        let body_res = self.body.check(env)?;
        let body_ty = body_res.ty();
        self.annot.check_equal(&body_ty)?;
        Ok(DefinitionDerivation::new(&self.name, body_res))
    }
}
