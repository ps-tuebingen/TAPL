use super::SolveState;
use crate::generate_constraints::constraints::SubtypeConstraint;
use syntax::language::Language;

pub fn solve_subtyping<Lang>(sub: SubtypeConstraint<Lang>, state: &mut SolveState<Lang>)
where
    Lang: Language,
{
    todo!()
}
