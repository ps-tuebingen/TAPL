use super::{Constraint, GenState, GenerateConstraints};
use std::collections::HashMap;
use syntax::{Name, language::Language, program::Program};

pub fn generate_constraints_program<Lang>(
    prog: &Program<Lang>,
) -> HashMap<Name, Vec<Constraint<Lang>>>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang>,
    Lang::Type: GenerateConstraints<Lang = Lang>,
{
    let mut constraint_map = HashMap::new();
    let mut state = GenState::new();
    prog.main.generate_constraints(&mut state);
    constraint_map.insert("main".to_string(), state.clear_constraints());
    for def in prog.definitions.iter() {
        def.generate_constraints(&mut state);
        constraint_map.insert(def.name.clone(), state.clear_constraints());
    }
    constraint_map
}
