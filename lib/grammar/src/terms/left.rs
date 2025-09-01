use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    language::Language,
    {terms::Left, },
};

impl<Lang> RuleDescribe for Left<Lang>
where
    Lang: Language,
    ,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Left, Some(Symbol::Type), vec![Symbol::Term]),
            "Left Injection",
        )
    }
}
