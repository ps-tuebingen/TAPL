use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::TryWithVal};

impl<Lang> GrammarRuleDescribe for TryWithVal<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
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
