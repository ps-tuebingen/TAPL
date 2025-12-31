pub mod constraints;
mod generate_constraints;
mod solve_constraints;

pub use constraints::{DefConstraints, ProgramConstraints};
pub use generate_constraints::{GenState, GenerateConstraints, generate_constraints_program};
pub use solve_constraints::{DefSubst, ProgSubst, SolveConstraint, SolveState, solve_constraints};
use std::{collections::HashMap, fmt};
use syntax::{Name, language::Language};

#[derive(Clone, Debug)]
pub struct ProgTypes<Lang>
where
    Lang: Language,
{
    pub main_ty: Lang::Type,
    pub def_tys: HashMap<Name, Lang::Type>,
}

impl<Lang> ProgTypes<Lang>
where
    Lang: Language,
{
    pub fn from_subst(substs: ProgSubst<Lang>) -> Self {
        let mut def_tys = HashMap::new();
        for (name, subst) in substs.def_substs {
            let ty = subst.apply();
            def_tys.insert(name, ty);
        }
        let main_ty = substs.main_subst.apply();
        Self { main_ty, def_tys }
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
