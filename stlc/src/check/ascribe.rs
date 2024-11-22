use super::{errors::Error, Check, TypingEnv};
use crate::{terms::syntax::Ascribe, types::Type};

impl Check for Ascribe {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let ty1 = self.term.check(env)?;
        if self.ty == ty1 {
            Ok(ty1)
        } else {
            Err(Error::WrongAscription {
                found: ty1,
                expected: self.ty.clone(),
            })
        }
    }
}
