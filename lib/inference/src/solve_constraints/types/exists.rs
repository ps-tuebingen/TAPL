use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    span::Spanned,
    subst::SubstType,
    types::{Exists, TypeGroup, TypeVariable},
};

impl<Lang> SolveConstraint for Exists<Lang>
where
    Lang: Language,
    Lang::Type: TypeGroup<Lang = Lang>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    fn solve_equality(
        self,
        rhs: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let err = TypeMismatch::new(rhs.to_string(), "Existential Type".to_string(), rhs.span());
        let rhs_exists = rhs.into_exists().ok_or(err)?;
        let rhs_subst = rhs_exists.ty.subst_type(
            &rhs_exists.var,
            &TypeVariable::new(&self.var, self.span).into(),
        );
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(rhs_subst),
            Rc::unwrap_or_clone(self.ty),
        ));
        Ok(())
    }

    fn solve_subtyping(
        self,
        sup: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        if sup.clone().into_top().is_some() {
            return Ok(());
        }

        let err = TypeMismatch::new(sup.to_string(), "Existential Type".to_string(), sup.span());
        let sup_exists = sup.into_exists().ok_or(err)?;
        let sup_subst = sup_exists.ty.subst_type(
            &sup_exists.var,
            &TypeVariable::new(&self.var, self.span).into(),
        );
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.ty),
            Rc::unwrap_or_clone(sup_subst),
        ));
        Ok(())
    }
}
