use crate::{
    GrammarRule, GrammarRuleDescribe, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Right};

impl<Lang> GrammarRuleDescribe for Right<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                Keyword::Right.into(),
                Symbol::sqbrack(Symbol::Type),
                SpecialChar::SqBrackC.into(),
                Symbol::paren(vec![Symbol::Term]),
            ]
            .into(),
            "Right Injection",
        )
    }
}
