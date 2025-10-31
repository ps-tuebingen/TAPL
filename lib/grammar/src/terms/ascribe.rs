use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, terms::Ascribe};

impl<Lang> GrammarRuleDescribe for Ascribe<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Symbol::colon_sep(Symbol::Term, Symbol::Type), "Ascription")
    }
}
