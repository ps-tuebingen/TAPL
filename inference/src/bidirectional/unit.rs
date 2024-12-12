use super::{errors::Error, Environment, Infer};
use crate::{syntax::Unit, types::Type};

impl Infer for Unit {
    fn infer(&self, _: &mut Environment) -> Result<Type, Error> {
        Ok(Type::Unit)
    }

    fn check(&self, target: Type, _: &mut Environment) -> Result<(), Error> {
        if Type::Unit == target {
            Ok(())
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::{Infer, Unit};
    use crate::types::Type;
    use std::collections::HashMap;

    #[test]
    fn infer_unit() {
        let result = Unit.infer(&mut HashMap::new()).unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_unit() {
        Unit.check(Type::Unit, &mut HashMap::new()).unwrap()
    }
}
