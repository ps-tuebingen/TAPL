use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Nothing};

impl<Lang> GrammarRuleDescribe for Nothing<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                Keyword::Nothing.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(vec![Symbol::Term]),
            ]
            .into(),
            "Nothing",
        )
    }
}
