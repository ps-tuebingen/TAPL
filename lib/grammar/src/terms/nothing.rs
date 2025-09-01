use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Nothing};

impl<Lang> RuleDescribe for Nothing<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Nothing, Some(Symbol::Type), vec![Symbol::Term]),
            "Nothing",
        )
    }
}
