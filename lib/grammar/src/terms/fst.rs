use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Fst};

impl<Lang> RuleDescribe for Fst<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(Keyword::Fst.into()), "First Projection")
    }
}
