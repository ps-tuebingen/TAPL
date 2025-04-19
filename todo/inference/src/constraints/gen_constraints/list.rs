use super::{GenConstraints, GenState};
use crate::{
    syntax::{Cons, Head, IsNil, Nil, Tail},
    types::Type,
};

impl GenConstraints for Nil {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let fresh_var = st.fresh_var();
        fresh_var.into()
    }
}

impl GenConstraints for Cons {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let fst_ty = self.fst.gen_constraints(st);
        let rst_ty = self.rst.gen_constraints(st);
        let list_ty = Type::List(Box::new(fst_ty));
        st.add_constraint(rst_ty, list_ty.clone());
        list_ty
    }
}

impl GenConstraints for IsNil {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let list_ty = self.list.gen_constraints(st);
        let fresh_var = st.fresh_var();
        st.add_constraint(list_ty, Type::List(Box::new(fresh_var.into())));
        Type::Bool
    }
}

impl GenConstraints for Head {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let list_ty = self.list.gen_constraints(st);
        let fresh_var = st.fresh_var();
        st.add_constraint(list_ty, Type::List(Box::new(fresh_var.clone().into())));
        fresh_var.into()
    }
}

impl GenConstraints for Tail {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let list_ty = self.list.gen_constraints(st);
        let fresh_var = st.fresh_var();
        let ret_ty = Type::List(Box::new(fresh_var.into()));
        st.add_constraint(list_ty, ret_ty.clone());
        ret_ty
    }
}

#[cfg(test)]
mod list_tests {
    use super::{Cons, GenConstraints, GenState, Head, IsNil, Nil, Tail};
    use crate::{syntax::Zero, types::Type};

    #[test]
    fn gen_nil() {
        let mut st = GenState::default();
        let result = Nil.gen_constraints(&mut st);
        let expected = Type::Var("X0".to_owned());
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_cons() {
        let mut st = GenState::default();
        let result = Cons {
            fst: Box::new(Zero.into()),
            rst: Box::new(Nil.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::List(Box::new(Type::Nat));
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        new_st.add_constraint("X0".to_owned().into(), Type::List(Box::new(Type::Nat)));
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_isnil() {
        let mut st = GenState::default();
        let result = IsNil {
            list: Box::new(Nil.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Bool;
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        new_st.used_vars.insert("X1".to_owned());
        new_st.add_constraint(
            "X0".to_owned().into(),
            Type::List(Box::new("X1".to_owned().into())),
        );
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_head() {
        let mut st = GenState::default();
        let result = Head {
            list: Box::new(Nil.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Var("X1".to_owned());
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        new_st.used_vars.insert("X1".to_owned());
        new_st.add_constraint(
            "X0".to_owned().into(),
            Type::List(Box::new("X1".to_owned().into())),
        );
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_tail() {
        let mut st = GenState::default();
        let result = Tail {
            list: Box::new(Nil.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::List(Box::new("X1".to_owned().into()));
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        new_st.used_vars.insert("X1".to_owned());
        new_st.add_constraint(
            "X0".to_owned().into(),
            Type::List(Box::new("X1".to_owned().into())),
        );
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
