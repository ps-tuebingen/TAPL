use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Fst};

impl<Lang> GrammarRuleDescribe for Fst<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(Keyword::Fst.into()), "First Projection")
    }
}
