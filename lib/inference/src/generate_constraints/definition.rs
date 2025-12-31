use super::{GenState, GenerateConstraints};
use crate::constraints::DefConstraints;
use std::collections::HashSet;
use syntax::{definition::Definition, free_vars::FreeTypeVars, language::Language};

pub fn generate_constraints_def<Lang>(def: &Definition<Lang>) -> DefConstraints<Lang>
where
    Lang: Language,
    Lang::Term: FreeTypeVars + GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Lang::Type: FreeTypeVars,
{
    let mut used = HashSet::new();
    def.body.free_type_vars(&mut used);
    def.annot.free_type_vars(&mut used);
    let mut state = GenState::new(used);
    let body_ty = def.body.generate_constraints(&mut state);
    DefConstraints {
        constraints: state.constraints,
        ret_ty: body_ty,
        used_type_vars: state.used_type_vars,
    }
}
