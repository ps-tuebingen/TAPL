use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    language::Language,
    {terms::Nil, },
};

impl<Lang> RuleDescribe for Nil<Lang>
where
    Lang: Language,
    ,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Nil, Some(Symbol::Term), vec![Symbol::Term]),
            "Empty List",
        )
    }
}
