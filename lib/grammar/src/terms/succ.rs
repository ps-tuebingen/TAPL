use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Succ};

impl<Lang> RuleDescribe for Succ<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Succ, None, vec![Symbol::Term]),
            "Succ",
        )
    }
}
