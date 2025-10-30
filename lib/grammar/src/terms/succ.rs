use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Succ};

impl<Lang> GrammarRuleDescribe for Succ<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Keyword::Succ.into(), Symbol::paren(Symbol::Term)].into(),
            "Succ",
        )
    }
}
