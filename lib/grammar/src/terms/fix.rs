use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Fix};

impl<Lang> GrammarRuleDescribe for Fix<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Fix, None, vec![Symbol::Term]),
            "Fixed Point",
        )
    }
}
