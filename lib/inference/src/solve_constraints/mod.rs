use crate::generate_constraints::Constraint;
use errors::inference_error::InferenceError;
use std::collections::HashMap;
use syntax::{Name, TypeVar, language::Language};

mod equality;
mod indexing;
mod kinding;
mod record;
mod subtyping;
mod variant;
use equality::solve_equality;
use indexing::solve_indexing;
use kinding::solve_kinding;
use record::solve_record;
use subtyping::solve_subtyping;
use variant::solve_variant;

pub struct VarSubst<Lang>
where
    Lang: Language,
{
    ty_vars: HashMap<TypeVar, Lang::Type>,
}

pub struct SolveState<Lang>
where
    Lang: Language,
{
    remaining_constraints: Vec<Constraint<Lang>>,
    var_tys: HashMap<TypeVar, Lang::Type>,
}

pub fn solve_constraints<Lang>(
    constraints: HashMap<Name, Vec<Constraint<Lang>>>,
) -> Result<HashMap<Name, VarSubst<Lang>>, InferenceError>
where
    Lang: Language,
{
    let mut res_map = HashMap::new();
    for (def_name, def_constraints) in constraints {
        let def_res = solve_def(def_constraints)?;
        res_map.insert(def_name, def_res);
    }
    Ok(res_map)
}

fn solve_def<Lang>(def_constraints: Vec<Constraint<Lang>>) -> Result<VarSubst<Lang>, InferenceError>
where
    Lang: Language,
{
    let mut state = SolveState {
        remaining_constraints: def_constraints,
        var_tys: HashMap::new(),
    };
    while let Some(constraint) = state.remaining_constraints.pop() {
        solve_constraint(constraint, &mut state);
    }

    Ok(VarSubst {
        ty_vars: state.var_tys,
    })
}

pub fn solve_constraint<Lang>(constraint: Constraint<Lang>, state: &mut SolveState<Lang>)
where
    Lang: Language,
{
    match constraint {
        Constraint::Equality(eq) => solve_equality(eq, state),
        Constraint::Subtyping(sub) => solve_subtyping(sub, state),
        Constraint::Kinding(knd) => solve_kinding(knd, state),
        Constraint::Indexing(ind) => solve_indexing(ind, state),
        Constraint::Record(rec) => solve_record(rec, state),
        Constraint::Variant(var) => solve_variant(var, state),
    }
}
