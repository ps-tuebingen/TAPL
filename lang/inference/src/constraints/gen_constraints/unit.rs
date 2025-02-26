use super::{GenConstraints, GenState};
use crate::{syntax::Unit, types::Type};

impl GenConstraints for Unit {
    fn gen_constraints(self, _: &mut GenState) -> Type {
        Type::Unit
    }
}

#[cfg(test)]
mod unit_tests {
    use super::{GenConstraints, GenState, Unit};
    use crate::types::Type;

    #[test]
    fn gen_unit() {
        let mut st = GenState::default();
        let result = Unit.gen_constraints(&mut st);
        let expected = Type::Unit;
        assert_eq!(result, expected);
        assert_eq!(st, GenState::default())
    }
}
