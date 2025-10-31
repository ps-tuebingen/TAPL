use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Try};

impl<Lang> GrammarRuleDescribe for Try<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Keyword::Try.into(), Symbol::brack(Symbol::Term)].into(),
            "Variant Case",
        )
    }
}
