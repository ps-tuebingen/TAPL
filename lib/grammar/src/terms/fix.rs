use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Fix};

impl<Lang> RuleDescribe for Fix<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Fix, None, vec![Symbol::Term]),
            "Fixed Point",
        )
    }
}
