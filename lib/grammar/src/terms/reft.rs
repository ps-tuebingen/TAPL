use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Ref};

impl<Lang> GrammarRuleDescribe for Ref<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Keyword::Ref.into(), Symbol::paren(Symbol::Term)].into(),
            "Reference",
        )
    }
}
