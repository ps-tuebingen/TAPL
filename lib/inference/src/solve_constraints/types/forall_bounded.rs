use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    span::Spanned,
    subst::SubstType,
    types::{ForallBounded, TypeGroup, TypeVariable},
};

impl<Lang> SolveConstraint for ForallBounded<Lang>
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
        let err = TypeMismatch::new(
            rhs.to_string(),
            "Universal Type (Bounded)".to_string(),
            rhs.span(),
        );
        let rhs_forall = rhs.into_forall_bounded().ok_or(err)?;
        let rhs_subst = rhs_forall.ty.subst_type(
            &rhs_forall.var,
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

        let err = TypeMismatch::new(
            sup.to_string(),
            "Universal Type (Bounded)".to_string(),
            sup.span(),
        );
        let sup_forall = sup.into_forall_bounded().ok_or(err)?;
        let sup_subst = sup_forall.ty.subst_type(
            &sup_forall.var,
            &TypeVariable::new(&self.var, self.span).into(),
        );
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.sup_ty.clone()),
            Rc::unwrap_or_clone(sup_forall.sup_ty),
        ));
        state.add_tyvar(&self.var, Rc::unwrap_or_clone(self.sup_ty));
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.ty),
            Rc::unwrap_or_clone(sup_subst),
        ));
        Ok(())
    }
}
