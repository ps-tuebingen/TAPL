use super::SolveState;
use crate::generate_constraints::constraints::RecordConstraint;
use syntax::language::Language;

pub fn solve_record<Lang>(rec: RecordConstraint<Lang>, state: &mut SolveState<Lang>)
where
    Lang: Language,
{
    todo!()
}
