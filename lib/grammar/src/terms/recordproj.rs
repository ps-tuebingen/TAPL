use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::RecordProj};

impl<Lang> GrammarRuleDescribe for RecordProj<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(Symbol::Label), "Record Projection")
    }
}
