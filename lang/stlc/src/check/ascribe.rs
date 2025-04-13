use super::{to_check_err, TypingEnv};
use crate::{syntax::Ascribe, types::Type};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Ascribe {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let ty1 = self.term.check(env)?;
        if self.ty == ty1 {
            Ok(ty1)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: ty1.to_string(),
                expected: self.ty.to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod ascribe_tests {
    use super::Ascribe;
    use crate::{syntax::Zero, types::Type};
    use common::Typecheck;

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
