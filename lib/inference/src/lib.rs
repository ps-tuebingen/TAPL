pub mod constraints;
mod generate_constraints;
mod solve_constraints;

use errors::inference_error::InferenceError;
use generate_constraints::{GenerateConstraints, generate_constraints_program};
use solve_constraints::{SolveConstraint, solve_constraints};
use std::collections::HashMap;
use syntax::{Name, language::Language, program::Program, subst::SubstType};

pub fn infer_types<Lang>(prog: Program<Lang>) -> Result<HashMap<Name, Lang::Type>, InferenceError>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Lang::Type: GenerateConstraints<Lang = Lang> + SolveConstraint<Lang = Lang>,
{
    let constraints = generate_constraints_program(&prog);
    let substs = solve_constraints(constraints)?;

    let mut tys = HashMap::new();
    for subst in substs {
        let mut ty_subst = subst.ty_no_subst;
        for (var, ty) in subst.ty_vars.iter() {
            ty_subst = ty_subst.subst_type(var, ty);
        }
        tys.insert(subst.name, ty_subst);
    }
    Ok(tys)
}
