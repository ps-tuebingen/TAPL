use crate::{errors::CheckError, Typecheck};
use common::errors::TypeMismatch;
use derivation::{DefinitionDerivation, ProgramDerivation, TypingDerivation};
use syntax::{
    env::Environment,
    program::{Definition, Program},
    terms::Term,
    types::Type,
};

impl<T, Ty> Typecheck for Program<T, Ty>
where
    T: Term + Typecheck<Term = T, Type = Ty, Deriv = TypingDerivation<T, Ty>>,
    Ty: Type,
{
    type Term = T;
    type Type = Ty;
    type Deriv = ProgramDerivation<T, Ty>;

    fn check(&self, env: Environment<Ty>) -> Result<Self::Deriv, CheckError<Ty>> {
        let mut deriv = ProgramDerivation::new();
        for def in self.definitions.iter() {
            let def_res = def.check(env.clone())?;
            deriv.def_derivations.push(def_res);
        }

        if let Some(ref mn) = self.main {
            let main_res = mn.check(env)?;
            deriv.main_derivation = Some(main_res)
        }
        Ok(deriv)
    }
}

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
