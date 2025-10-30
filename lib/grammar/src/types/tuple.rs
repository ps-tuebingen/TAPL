use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::Tuple};

impl<Lang> GrammarRuleDescribe for Tuple<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::paren(Symbol::many(Symbol::Type)), "Tuple Type")
    }
}
