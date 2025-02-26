use super::{GenConstraints, GenState};
use crate::{
    constraints::constraint::Constraint,
    syntax::{Proj, Tup},
    types::Type,
};

impl GenConstraints for Tup {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let mut tys = vec![];
        for term in self.terms.into_iter() {
            let term_ty = term.gen_constraints(st);
            tys.push(term_ty)
        }
        Type::Tup(tys)
    }
}

impl GenConstraints for Proj {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let tup_ty = self.tup.gen_constraints(st);
        let fresh_var = st.fresh_var();
        st.constrs.push(Constraint::TupleConstraint {
            tup_ty,
            ind: self.ind,
            ind_ty: fresh_var.clone().into(),
        });
        fresh_var.into()
    }
}
