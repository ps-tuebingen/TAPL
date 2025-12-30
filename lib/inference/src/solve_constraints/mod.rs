use crate::{constraints::Constraint, generate_constraints::DefConstraints};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::collections::HashMap;
use syntax::{Label, Name, TypeVar, language::Language, types::Type};

mod kinding;
mod types;
mod untyped;

mod state;

use kinding::solve_kinding;
pub use state::SolveState;

pub struct DefSubst<Lang>
where
    Lang: Language,
{
    pub name: Name,
    pub ty_vars: HashMap<TypeVar, Lang::Type>,
    pub ty_no_subst: Lang::Type,
}

pub fn solve_constraints<Lang>(
    constraints: Vec<DefConstraints<Lang>>,
) -> Result<Vec<DefSubst<Lang>>, InferenceError>
where
    Lang: Language,
    Lang::Type: SolveConstraint<Lang = Lang>,
{
    let mut substitutions = Vec::with_capacity(constraints.len());
    for constrs in constraints {
        substitutions.push(solve_def(constrs)?);
    }
    Ok(substitutions)
}

fn solve_def<Lang>(def_constraints: DefConstraints<Lang>) -> Result<DefSubst<Lang>, InferenceError>
where
    Lang: Language,
    Lang::Type: SolveConstraint<Lang = Lang>,
{
    let mut state = SolveState::new(def_constraints.constraints);
    while let Some(constraint) = state.next_constraint() {
        solve_constraint(constraint, &mut state)?;
    }
    Ok(DefSubst {
        name: def_constraints.name,
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

pub trait SolveConstraint: Type {
    type Lang: Language;

    fn solve_equality(
        self,
        rhs: <Self::Lang as Language>::Type,
        state: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError>;

    fn solve_subtyping(
        self,
        sup: <Self::Lang as Language>::Type,
        state: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError>;

    fn solve_index(
        self,
        _ind: usize,
        _ind_ty: <Self::Lang as Language>::Type,
        _state: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError> {
        Err(TypeMismatch::new(self.to_string(), "Tuple Type".to_string(), self.span()).into())
    }

    fn solve_record(
        self,
        _: Label,
        _: <Self::Lang as Language>::Type,
        _: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError> {
        Err(TypeMismatch::new(self.to_string(), "Record Type".to_string(), self.span()).into())
    }

    fn solve_variant(
        self,
        _: Label,
        _: <Self::Lang as Language>::Type,
        _: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError> {
        Err(TypeMismatch::new(self.to_string(), "Variant Type".to_string(), self.span()).into())
    }
}
