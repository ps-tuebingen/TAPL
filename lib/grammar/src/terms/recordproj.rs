use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::RecordProj};

impl<Lang> RuleDescribe for RecordProj<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(Symbol::Label), "Record Projection")
    }
}
