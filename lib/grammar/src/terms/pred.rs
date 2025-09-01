use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Pred};

impl<Lang> RuleDescribe for Pred<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Pred, None, vec![Symbol::Term]),
            "Pred",
        )
    }
}
