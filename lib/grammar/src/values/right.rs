use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Right};

impl<Lang> RuleDescribe for Right<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Right, Some(Symbol::Type), vec![Symbol::Value]),
            "Right Injection",
        )
    }
}
