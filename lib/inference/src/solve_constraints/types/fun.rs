use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    span::Spanned,
    types::{Fun, TypeGroup, TypeVariable},
};

impl<Lang> SolveConstraint for Fun<Lang>
where
    Lang: Language,
    TypeVariable<Lang>: Into<Lang::Type>,
    Fun<Lang>: Into<Lang::Type>,
    Lang::Type: TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn solve_equality(
        self,
        rhs: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        if let Some(v) = rhs.clone().into_variable() {
            let from_var = state.fresh_type_var();
            let from_ty = TypeVariable::new(&from_var, self.span);
            let to_var = state.fresh_type_var();
            let to_ty = TypeVariable::new(&to_var, self.span);
            state.add_constraint(EqualityConstraint::new(
                Fun::new(from_ty.clone(), to_ty.clone(), self.span),
                v,
            ));
            state.add_constraint(EqualityConstraint::new(
                Rc::unwrap_or_clone(self.from),
                from_ty,
            ));
            state.add_constraint(EqualityConstraint::new(Rc::unwrap_or_clone(self.to), to_ty));
            return Ok(());
        }

        let err = TypeMismatch::new(rhs.to_string(), "Function Type".to_string(), rhs.span());
        let rhs_fun = rhs.into_fun().ok_or(err)?;
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(self.from),
            Rc::unwrap_or_clone(rhs_fun.from),
        ));
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(self.to),
            Rc::unwrap_or_clone(rhs_fun.to),
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

        if let Some(v) = sup.clone().into_variable() {
            let from_var = state.fresh_type_var();
            let from_ty = TypeVariable::new(&from_var, self.span);
            let to_var = state.fresh_type_var();
            let to_ty = TypeVariable::new(&to_var, self.span);
            state.add_constraint(SubtypeConstraint::new(
                from_ty.clone(),
                Rc::unwrap_or_clone(self.from),
            ));
            state.add_constraint(SubtypeConstraint::new(
                Rc::unwrap_or_clone(self.to),
                to_ty.clone(),
            ));
            state.add_constraint(EqualityConstraint::new(
                Fun::new(from_ty, to_ty, self.span),
                v,
            ));
            return Ok(());
        }

        let err = TypeMismatch::new(sup.to_string(), "Function Type".to_string(), sup.span());
        let sup_fun = sup.into_fun().ok_or(err)?;
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(sup_fun.from),
            Rc::unwrap_or_clone(self.from),
        ));
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.to),
            Rc::unwrap_or_clone(sup_fun.to),
        ));

        Ok(())
    }
}
