use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Fold};

impl<Lang> RuleDescribe for Fold<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Fold, Some(Symbol::Type), vec![Symbol::Term]),
            "Fold",
        )
    }
}
