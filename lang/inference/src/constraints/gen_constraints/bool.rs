use super::{GenConstraints, GenState};
use crate::{
    syntax::{False, If, True},
    types::Type,
};

impl GenConstraints for True {
    fn gen_constraints(self, _: &mut GenState) -> Type {
        Type::Bool
    }
}

impl GenConstraints for False {
    fn gen_constraints(self, _: &mut GenState) -> Type {
        Type::Bool
    }
}

impl GenConstraints for If {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let cond_ty = self.ifc.gen_constraints(st);
        st.add_constraint(cond_ty, Type::Bool);
        let then_ty = self.thenc.gen_constraints(st);
        let else_ty = self.elsec.gen_constraints(st);
        st.add_constraint(then_ty.clone(), else_ty);
        then_ty
    }
}

#[cfg(test)]
mod bool_tests {
    use super::{False, GenConstraints, GenState, If, True};
    use crate::{syntax::Zero, types::Type};

    #[test]
    fn gen_true() {
        let mut st = GenState::default();
        let result = True.gen_constraints(&mut st);
        let expected = Type::Bool;
        assert_eq!(result, expected);
        assert_eq!(st, GenState::default())
    }

    #[test]
    fn gen_false() {
        let mut st = GenState::default();
        let result = False.gen_constraints(&mut st);
        let expected = Type::Bool;
        assert_eq!(result, expected);
        assert_eq!(st, GenState::default());
    }

    #[test]
    fn gen_if() {
        let mut st = GenState::default();
        let result = If {
            ifc: Box::new(True.into()),
            thenc: Box::new(Zero.into()),
            elsec: Box::new(Zero.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Nat;
        let mut new_st = GenState::default();
        new_st.add_constraint(Type::Bool, Type::Bool);
        new_st.add_constraint(Type::Nat, Type::Nat);
        assert_eq!(result, expected);
        assert_eq!(st, new_st);
    }
}
