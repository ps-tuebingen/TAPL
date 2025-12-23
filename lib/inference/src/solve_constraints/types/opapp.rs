use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    subst::SubstType,
    types::{OpApp, TypeGroup},
};

impl<Lang> SolveConstraint for OpApp<Lang>
where
    Lang: Language,
    Lang::Type: TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn solve_equality(
        self,
        rhs: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let oplam = Rc::unwrap_or_clone(self.fun.clone()).into_oplambda();
        if let Some(oplam) = oplam {
            let body_subst = oplam.body.subst_type(&oplam.var, &self.arg);
            state.add_constraint(EqualityConstraint::new(
                Rc::unwrap_or_clone(body_subst),
                rhs,
            ));
            return Ok(());
        }

        let oplam_sub = Rc::unwrap_or_clone(self.fun.clone()).into_oplambdasub();
        if let Some(oplam) = oplam_sub {
            let body_subst = oplam.body.subst_type(&oplam.var, &self.arg);
            state.add_constraint(EqualityConstraint::new(
                Rc::unwrap_or_clone(body_subst),
                rhs,
            ));
            return Ok(());
        }

        let rhs_app = rhs.clone().into_opapp();
        if let Some(app) = rhs_app {
            state.add_constraint(EqualityConstraint::new(
                Rc::unwrap_or_clone(self.fun),
                Rc::unwrap_or_clone(app.fun),
            ));
            state.add_constraint(EqualityConstraint::new(
                Rc::unwrap_or_clone(self.arg),
                Rc::unwrap_or_clone(app.arg),
            ));
            return Ok(());
        }

        Err(TypeMismatch::new(self.to_string(), rhs.to_string(), self.span).into())
    }

    fn solve_subtyping(
        self,
        sup: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let oplam = Rc::unwrap_or_clone(self.fun.clone()).into_oplambda();
        if let Some(oplam) = oplam {
            let body_subst = oplam.body.subst_type(&oplam.var, &self.arg);
            state.add_constraint(SubtypeConstraint::new(Rc::unwrap_or_clone(body_subst), sup));
            return Ok(());
        }

        let oplam_sub = Rc::unwrap_or_clone(self.fun.clone()).into_oplambdasub();
        if let Some(oplam) = oplam_sub {
            let body_subst = oplam.body.subst_type(&oplam.var, &self.arg);
            state.add_constraint(SubtypeConstraint::new(Rc::unwrap_or_clone(body_subst), sup));
            return Ok(());
        }

        let rhs_app = sup.clone().into_opapp();
        if let Some(app) = rhs_app {
            state.add_constraint(SubtypeConstraint::new(
                Rc::unwrap_or_clone(self.fun),
                Rc::unwrap_or_clone(app.fun),
            ));
            state.add_constraint(SubtypeConstraint::new(
                Rc::unwrap_or_clone(self.arg),
                Rc::unwrap_or_clone(app.arg),
            ));
            return Ok(());
        }

        Err(TypeMismatch::new(self.to_string(), sup.to_string(), self.span).into())
    }
}
