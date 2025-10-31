use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::TryWithVal};

impl<Lang> GrammarRuleDescribe for TryWithVal<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                Keyword::Try.into(),
                Symbol::brack(Symbol::Term),
                Keyword::Catch.into(),
                Symbol::brack(Symbol::Term),
            ]
            .into(),
            "Try-Catch",
        )
    }
}
