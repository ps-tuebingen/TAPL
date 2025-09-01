use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    language::Language,
    {terms::Tail, },
};

impl<Lang> RuleDescribe for Tail<Lang>
where
    Lang: Language,
    ,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Tail, Some(Symbol::Type), vec![Symbol::Term]),
            "List Tail",
        )
    }
}
