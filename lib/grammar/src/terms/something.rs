use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Something};

impl<Lang> GrammarRuleDescribe for Something<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Something, None, vec![Symbol::Term]),
            "Something",
        )
    }
}
