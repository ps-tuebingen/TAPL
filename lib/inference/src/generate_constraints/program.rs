use super::{GenState, GenerateConstraints, definition::generate_constraints_def};
use crate::{ProgramConstraints, constraints::DefConstraints};
use std::collections::{HashMap, HashSet};
use syntax::{free_vars::FreeTypeVars, language::Language, program::Program};

pub fn generate_constraints_program<Lang>(prog: &Program<Lang>) -> ProgramConstraints<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Lang::Type: GenerateConstraints<Lang = Lang>,
{
    let mut def_constraints = HashMap::new();

    let mut used = HashSet::new();
    prog.main.free_type_vars(&mut used);
    let mut main_state = GenState::new(used);
    let main_ty = prog.main.generate_constraints(&mut main_state);
    let main_constraints = DefConstraints {
        constraints: main_state.constraints,
        ret_ty: main_ty,
        used_type_vars: main_state.used_type_vars,
    };

    for def in &prog.definitions {
        def_constraints.insert(def.name.clone(), generate_constraints_def(def));
    }
    ProgramConstraints {
        def_constraints,
        main_constraints,
    }
}
