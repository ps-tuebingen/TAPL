use super::SolveState;
use crate::generate_constraints::constraints::VariantConstraint;
use syntax::language::Language;

pub fn solve_variant<Lang>(var: VariantConstraint<Lang>, state: &mut SolveState<Lang>)
where
    Lang: Language,
{
    todo!()
}
