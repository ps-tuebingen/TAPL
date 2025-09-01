use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    language::Language,
    {terms::Unfold, },
};

impl<Lang> RuleDescribe for Unfold<Lang>
where
    Lang: Language,
    ,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Unfold, Some(Symbol::Type), vec![Symbol::Term]),
            "Unfold",
        )
    }
}
