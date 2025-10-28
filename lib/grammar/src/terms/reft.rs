use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Ref};

impl<Lang> GrammarRuleDescribe for Ref<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Ref, None, vec![Symbol::Term]),
            "Reference",
        )
    }
}
