use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    span::Spanned,
    subst::SubstType,
    types::{OpLambda, TypeGroup, TypeVariable},
};

impl<Lang> SolveConstraint for OpLambda<Lang>
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
            "Operator Abstraction".to_string(),
            rhs.span(),
        );
        let rhs_lam = rhs.into_oplambda().ok_or(err)?;
        let rhs_subst = rhs_lam.body.subst_type(
            &rhs_lam.var,
            &TypeVariable::new(&self.var, self.span).into(),
        );
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(self.body),
            Rc::unwrap_or_clone(rhs_subst),
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
            "Operator Abstraction".to_string(),
            self.span,
        );
        let sup_lam = sup.into_oplambda().ok_or(err)?;
        let sup_subst = sup_lam.body.subst_type(
            &sup_lam.var,
            &TypeVariable::new(&self.var, self.span).into(),
        );
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.body),
            Rc::unwrap_or_clone(sup_subst),
        ));
        Ok(())
    }
}
