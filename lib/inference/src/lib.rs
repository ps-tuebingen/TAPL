pub mod constraints;
mod generate_constraints;
mod solve_constraints;

use errors::{DefinitionNotFound, inference_error::InferenceError};
use generate_constraints::{GenerateConstraints, generate_constraints_program};
use solve_constraints::{SolveConstraint, solve_constraints};
use syntax::{language::Language, program::Program, subst::SubstType};

pub fn infer_types<Lang>(mut prog: Program<Lang>) -> Result<Program<Lang>, InferenceError>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang>,
    Lang::Type: GenerateConstraints<Lang = Lang> + SolveConstraint<Lang = Lang>,
{
    let constraints = generate_constraints_program(&prog);
    let subst = solve_constraints(constraints)?;
    let mut new_defs = Vec::with_capacity(prog.definitions.len());
    let mut new_main = prog.main;
    for (def_name, def_subst) in subst {
        if def_name == "main" {
            for (var, ty) in def_subst.ty_vars.iter() {
                new_main = new_main.subst_type(&var, &ty);
            }
            continue;
        }
        let def_ind = prog
            .definitions
            .iter()
            .position(|def| def.name == def_name)
            .ok_or(DefinitionNotFound::new(&def_name))?;
        let mut def = prog.definitions.remove(def_ind);
        for (var, ty) in def_subst.ty_vars.iter() {
            def = def.subst_type(&var, &ty);
        }
        new_defs.push(def);
    }
    Ok(Program::new(new_main, new_defs))
}
