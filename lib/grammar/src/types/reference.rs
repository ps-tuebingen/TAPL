use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, types::Reference};

impl<Lang> RuleDescribe for Reference<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Reference, Some(Symbol::Type), vec![]),
            "Reference Type",
        )
    }
}
