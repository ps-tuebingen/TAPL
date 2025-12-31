use crate::{
    ProgramConstraints,
    constraints::{Constraint, DefConstraints},
};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::collections::HashMap;
use syntax::{Label, language::Language, types::Type};

mod kinding;
mod state;
mod subst;
mod types;
mod untyped;

use kinding::solve_kinding;
pub use state::SolveState;
pub use subst::{DefSubst, ProgSubst};

/// Solve a set of program constraints
/// # Errors
/// Returns an error if a constraint could not be satisfied
pub fn solve_constraints<Lang>(
    constraints: ProgramConstraints<Lang>,
) -> Result<ProgSubst<Lang>, InferenceError>
where
    Lang: Language,
    Lang::Type: SolveConstraint<Lang = Lang>,
{
    let mut substitutions = HashMap::new();
    for (name, constrs) in constraints.def_constraints {
        let solved = solve_def(constrs)?;
        substitutions.insert(name, solved);
    }
    let main_subst = solve_def(constraints.main_constraints)?;
    Ok(ProgSubst {
        def_substs: substitutions,
        main_subst,
    })
}

fn solve_def<Lang>(def_constraints: DefConstraints<Lang>) -> Result<DefSubst<Lang>, InferenceError>
where
    Lang: Language,
    Lang::Type: SolveConstraint<Lang = Lang>,
{
    let mut state = SolveState::new(def_constraints.constraints, def_constraints.used_type_vars);
    while let Some(constraint) = state.next_constraint() {
        solve_constraint(constraint, &mut state)?;
    }
    Ok(DefSubst {
        ty_vars: state.var_tys,
        ty_no_subst: def_constraints.ret_ty,
    })
}

pub fn solve_constraint<Lang>(
    constraint: Constraint<Lang>,
    state: &mut SolveState<Lang>,
) -> Result<(), InferenceError>
where
    Lang: Language,
    Lang::Type: SolveConstraint<Lang = Lang>,
{
    match constraint {
        Constraint::Equality(eq) => eq.left.solve_equality(eq.right, state),
        Constraint::Subtyping(sub) => sub.sub_type.solve_subtyping(sub.super_type, state),
        Constraint::Kinding(knd) => solve_kinding(knd, state),
        Constraint::Indexing(ind) => ind.ty.solve_index(ind.ind, ind.ind_ty, state),
        Constraint::Record(rec) => rec.ty.solve_record(rec.label, rec.label_ty, state),
        Constraint::Variant(var) => var.ty.solve_variant(var.label, var.label_ty, state),
    }
}

/// Solve Typing Constraints for a given type
pub trait SolveConstraint: Type {
    /// Language this type belongs to
    type Lang: Language;

    /// Solve equality Constraint
    /// # Errors
    /// Returns an error if equality could not be satisfied
    fn solve_equality(
        self,
        rhs: <Self::Lang as Language>::Type,
        state: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError>;

    /// Solve subtyping constraint
    /// # Errors
    /// Returns an error if subtyping could not be satisfied
    fn solve_subtyping(
        self,
        sup: <Self::Lang as Language>::Type,
        state: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError>;

    /// Solve indexing constraint
    /// # Errors
    /// Returns an error if `Self` is not a tuple
    fn solve_index(
        self,
        _ind: usize,
        _ind_ty: <Self::Lang as Language>::Type,
        _state: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError> {
        Err(TypeMismatch::new(self.to_string(), "Tuple Type".to_string(), self.span()).into())
    }

    /// Solve record constraint
    /// # Errors
    /// Returns an error if `Self` is not a record type
    fn solve_record(
        self,
        _: Label,
        _: <Self::Lang as Language>::Type,
        _: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError> {
        Err(TypeMismatch::new(self.to_string(), "Record Type".to_string(), self.span()).into())
    }

    /// Solve a variant constraint
    /// # Errors
    /// Returns an error if `Self` is not a variant type
    fn solve_variant(
        self,
        _: Label,
        _: <Self::Lang as Language>::Type,
        _: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError> {
        Err(TypeMismatch::new(self.to_string(), "Variant Type".to_string(), self.span()).into())
    }
}
