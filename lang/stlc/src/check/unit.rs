use super::{errors::Error, Check, TypingEnv};
use crate::{syntax::Unit, types::Type};

impl Check for Unit {
    fn check(&self, _: &mut TypingEnv) -> Result<Type, Error> {
        Ok(Type::Unit)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::{Check, Unit};
    use crate::types::Type;

    #[test]
    fn check_unit() {
        let result = Unit.check(&mut Default::default()).unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }
}
