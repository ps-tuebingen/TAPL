use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Cast};

impl<Lang> GrammarRuleDescribe for Cast<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Term, Keyword::As.into(), Symbol::Term].into(),
            "Cast",
        )
    }
}
