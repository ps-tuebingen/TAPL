use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    span::Spanned,
    types::{Product, TypeGroup},
};

impl<Lang> SolveConstraint for Product<Lang>
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
        let err = TypeMismatch::new(rhs.to_string(), "Product Type".to_string(), rhs.span());
        let rhs_product = rhs.into_product().ok_or(err)?;
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(self.fst),
            Rc::unwrap_or_clone(rhs_product.fst),
        ));
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(self.snd),
            Rc::unwrap_or_clone(rhs_product.snd),
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

        let err = TypeMismatch::new(sup.to_string(), "Product Type".to_string(), sup.span());
        let sup_prod = sup.into_product().ok_or(err)?;
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.fst),
            Rc::unwrap_or_clone(sup_prod.fst),
        ));
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.snd),
            Rc::unwrap_or_clone(sup_prod.snd),
        ));
        Ok(())
    }
}
