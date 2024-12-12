use super::{GenConstraints, GenState};
use crate::{
    syntax::{Pair, Proj1, Proj2},
    types::Type,
};

impl GenConstraints for Pair {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let fst_ty = self.fst.gen_constraints(st);
        let snd_ty = self.snd.gen_constraints(st);
        Type::Prod(Box::new(fst_ty), Box::new(snd_ty))
    }
}

impl GenConstraints for Proj1 {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let pair_ty = self.pair.gen_constraints(st);
        let var_fst = st.fresh_var();
        let var_snd = st.fresh_var();
        let tup_ty = Type::Prod(Box::new(var_fst.clone().into()), Box::new(var_snd.into()));
        st.add_constraint(pair_ty, tup_ty);
        var_fst.into()
    }
}

impl GenConstraints for Proj2 {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let pair_ty = self.pair.gen_constraints(st);
        let var_fst = st.fresh_var();
        let var_snd = st.fresh_var();
        let tup_ty = Type::Prod(Box::new(var_fst.into()), Box::new(var_snd.clone().into()));
        st.add_constraint(pair_ty, tup_ty);
        var_snd.into()
    }
}

#[cfg(test)]
mod pair_tests {
    use super::{GenConstraints, GenState, Pair, Proj1, Proj2};
    use crate::{
        syntax::{False, True},
        types::Type,
    };

    #[test]
    fn gen_pair() {
        let mut st = GenState::default();
        let result = Pair {
            fst: Box::new(True.into()),
            snd: Box::new(False.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Prod(Box::new(Type::Bool), Box::new(Type::Bool));
        assert_eq!(result, expected);
        assert_eq!(st, GenState::default())
    }

    #[test]
    fn gen_proj1() {
        let mut st = GenState::default();
        let result = Proj1 {
            pair: Box::new("x".to_owned().into()),
        }
        .gen_constraints(&mut st);
        let expected = "X1".to_owned().into();
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        new_st.used_vars.insert("X1".to_owned());
        new_st.used_vars.insert("X2".to_owned());
        new_st.add_constraint(
            "X0".to_owned().into(),
            Type::Prod(
                Box::new("X1".to_owned().into()),
                Box::new("X2".to_owned().into()),
            ),
        );
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_proj2() {
        let mut st = GenState::default();
        let result = Proj2 {
            pair: Box::new("x".to_owned().into()),
        }
        .gen_constraints(&mut st);
        let expected = "X2".to_owned().into();
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        new_st.used_vars.insert("X1".to_owned());
        new_st.used_vars.insert("X2".to_owned());
        new_st.add_constraint(
            "X0".to_owned().into(),
            Type::Prod(
                Box::new("X1".to_owned().into()),
                Box::new("X2".to_owned().into()),
            ),
        );
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
