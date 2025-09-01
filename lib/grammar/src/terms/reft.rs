use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Ref};

impl<Lang> RuleDescribe for Ref<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Ref, None, vec![Symbol::Term]),
            "Reference",
        )
    }
}
