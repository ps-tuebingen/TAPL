use super::SolveState;
use crate::generate_constraints::constraints::KindConstraint;
use syntax::language::Language;

pub fn solve_kinding<Lang>(knd: KindConstraint, state: &mut SolveState<Lang>)
where
    Lang: Language,
{
    todo!()
}
