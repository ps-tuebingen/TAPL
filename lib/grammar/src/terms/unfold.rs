use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Unfold};

impl<Lang> GrammarRuleDescribe for Unfold<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Unfold, Some(Symbol::Type), vec![Symbol::Term]),
            "Unfold",
        )
    }
}
