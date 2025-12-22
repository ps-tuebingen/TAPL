use super::{Constraint, GenState, GenerateConstraints};
use std::{collections::HashMap, mem::replace};
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
    let mut state = GenState::new(&prog.main);
    prog.main.generate_constraints(&mut state);
    let constraints = replace(&mut state.constraints, Vec::new());
    constraint_map.insert("main".to_string(), constraints);

    for def in prog.definitions.iter() {
        state = GenState::new(&def.body);
        def.generate_constraints(&mut state);
        let constraints = replace(&mut state.constraints, Vec::new());
        constraint_map.insert(def.name.clone(), constraints);
    }
    constraint_map
}
