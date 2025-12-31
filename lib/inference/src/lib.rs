pub mod constraints;
mod generate_constraints;
mod solve_constraints;

use errors::{UndefinedMain, inference_error::InferenceError};
use generate_constraints::generate_constraints_program;
pub use generate_constraints::{GenState, GenerateConstraints};
use solve_constraints::solve_constraints;
pub use solve_constraints::{SolveConstraint, SolveState};
use std::{collections::HashMap, fmt};
use syntax::{Name, language::Language, program::Program, subst::SubstType};

#[derive(Clone, Debug)]
pub struct ProgTypes<Lang>
where
    Lang: Language,
{
    pub main_ty: Lang::Type,
    pub def_tys: HashMap<Name, Lang::Type>,
}

pub fn infer_types<Lang>(prog: &Program<Lang>) -> Result<ProgTypes<Lang>, InferenceError>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Lang::Type: GenerateConstraints<Lang = Lang> + SolveConstraint<Lang = Lang>,
{
    let constraints = generate_constraints_program(&prog);
    println!(
        "generated constraints:\n{:?}",
        constraints
            .iter()
            .map(|c| &c.constraints)
            .collect::<Vec<_>>()
    );
    let substs = solve_constraints(constraints)?;

    let mut tys = HashMap::new();
    let mut main_ty = None;
    for subst in substs {
        let mut ty_subst = subst.ty_no_subst;
        for (var, ty) in subst.ty_vars.iter() {
            ty_subst = ty_subst.subst_type(var, ty);
        }
        if subst.name == "main" {
            main_ty = Some(ty_subst);
        } else {
            tys.insert(subst.name, ty_subst);
        }
    }

    if let Some(ty) = main_ty {
        Ok(ProgTypes {
            main_ty: ty,
            def_tys: tys,
        })
    } else {
        Err(UndefinedMain.into())
    }
}

impl<Lang> fmt::Display for ProgTypes<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, ty) in &self.def_tys {
            writeln!(f, "{name} : {ty}")?;
        }
        writeln!(f, "main : {}", self.main_ty)
    }
}
