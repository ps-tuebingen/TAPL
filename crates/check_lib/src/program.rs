use crate::{errors::CheckError, Typecheck};
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
