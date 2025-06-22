use crate::{errors::CheckError, Typecheck};
use common::errors::TypeMismatch;
use derivation::{DefinitionDerivation, TypingDerivation};
use syntax::{definition::Definition, env::Environment, terms::Term, types::Type};

impl<T, Ty> Typecheck for Definition<T, Ty>
where
    T: Term + Typecheck<Term = T, Type = Ty, Deriv = TypingDerivation<T, Ty>>,
    Ty: Type,
{
    type Term = T;
    type Type = Ty;
    type Deriv = DefinitionDerivation<T, Ty>;
    fn check(&self, env: Environment<Ty>) -> Result<Self::Deriv, CheckError<Ty>> {
        let body_res = self.body.check(env)?;
        let body_ty = body_res.ty();
        if self.annot != body_ty {
            return Err(TypeMismatch::new(self.annot.knd(), body_ty.knd()).into());
        }
        Ok(DefinitionDerivation::new(&self.name, body_res))
    }
}
