use crate::{errors::CheckError, Typecheck};
use derivation::{ProgramDerivation, TypingDerivation};
use syntax::{
    env::Environment,
    program::Program,
    terms::Term,
    types::{Type, TypeGroup},
};

impl<T, Ty> Typecheck for Program<T, Ty>
where
    T: Term + Typecheck<Term = T, Type = Ty, Deriv = TypingDerivation<T, Ty>>,
    Ty: Type + TypeGroup,
{
    type Term = T;
    type Type = Ty;
    type Deriv = ProgramDerivation<T, Ty>;

    fn check(&self, mut env: Environment<Ty>) -> Result<Self::Deriv, CheckError> {
        let mut derivs = vec![];
        for def in self.definitions.iter() {
            env.add_definition(def.name.clone(), def.annot.clone());
        }
        for def in self.definitions.iter() {
            let def_res = def.check(env.clone())?;
            derivs.push(def_res);
        }

        let main_res = self.main.check(env)?;
        Ok(ProgramDerivation::new(main_res, derivs))
    }
}
