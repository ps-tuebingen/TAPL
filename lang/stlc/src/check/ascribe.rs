use super::{errors::Error, Check, TypingEnv};
use crate::{syntax::Ascribe, types::Type};

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

#[cfg(test)]
mod ascribe_tests {
    use super::{Ascribe, Check};
    use crate::{syntax::Zero, types::Type};

    #[test]
    fn check_ascribe() {
        let result = Ascribe {
            term: Box::new(Zero.into()),
            ty: Type::Nat,
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }
}
