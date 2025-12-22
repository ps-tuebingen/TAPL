use super::SolveState;
use crate::generate_constraints::constraints::IndexConstraint;
use syntax::language::Language;

pub fn solve_indexing<Lang>(ind: IndexConstraint<Lang>, state: &mut SolveState<Lang>)
where
    Lang: Language,
{
    todo!()
}
