use super::{DefConstraints, GenState, GenerateConstraints, definition::generate_constraints_def};
use std::collections::HashSet;
use syntax::{free_vars::FreeTypeVars, language::Language, program::Program};

pub fn generate_constraints_program<Lang>(prog: &Program<Lang>) -> Vec<DefConstraints<Lang>>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Lang::Type: GenerateConstraints<Lang = Lang>,
{
    let mut constraints = Vec::with_capacity(prog.definitions.len() + 1);

    let mut used = HashSet::new();
    prog.main.free_type_vars(&mut used);
    let mut main_state = GenState::new(used);
    let main_ty = prog.main.generate_constraints(&mut main_state);
    constraints.push(DefConstraints {
        name: "main".to_string(),
        constraints: main_state.constraints,
        ret_ty: main_ty,
    });
    for def in prog.definitions.iter() {
        constraints.push(generate_constraints_def(def));
    }
    constraints
}
