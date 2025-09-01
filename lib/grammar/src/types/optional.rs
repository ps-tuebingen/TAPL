use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, types::Optional};

impl<Lang> RuleDescribe for Optional<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Optional, Some(Symbol::Type), vec![]),
            "Option Type",
        )
    }
}
