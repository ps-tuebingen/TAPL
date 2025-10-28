use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, types::Source};

impl<Lang> GrammarRuleDescribe for Source<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Source, Some(Symbol::Type), vec![]),
            "Source Type",
        )
    }
}
