use super::TypingEnv;
use crate::{syntax::Unit, types::Type};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Unit {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        Ok(Type::Unit)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::Unit;
    use crate::types::Type;
    use common::Typecheck;

    #[test]
    fn check_unit() {
        let result = Unit.check(&mut Default::default()).unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }
}
