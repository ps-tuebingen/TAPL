use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Left};

impl<Lang> RuleDescribe for Left<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Left, Some(Symbol::Type), vec![Symbol::Value]),
            "Left Injection",
        )
    }
}
