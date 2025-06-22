use crate::{Typecheck, errors::CheckError};
use derivation::{ProgramDerivation, TypingDerivation};
use syntax::{env::Environment, program::Program, terms::Term, types::Type};

impl<T, Ty> Typecheck for Program<T, Ty>
where
    T: Term + Typecheck<Term = T, Type = Ty, Deriv = TypingDerivation<T, Ty>>,
    Ty: Type,
{
    type Term = T;
    type Type = Ty;
    type Deriv = ProgramDerivation<T, Ty>;

    fn check(&self, mut env: Environment<Ty>) -> Result<Self::Deriv, CheckError> {
        let mut derivs = vec![];
        for def in self.definitions.iter() {
            let def_res = def.check(env.clone())?;
            env.add_var(def.name.clone(), def_res.ty());
            derivs.push(def_res);
        }

        let main_res = self.main.check(env)?;
        Ok(ProgramDerivation::new(main_res, derivs))
    }
}
