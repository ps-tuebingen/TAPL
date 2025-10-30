use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, values::Cons};

impl<Lang> GrammarRuleDescribe for Cons<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Cons.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(vec![
                    Symbol::Value,
                    SpecialChar::Comma.into(),
                    Symbol::Value,
                ]),
            ]
            .into(),
            "Cons",
        )
    }
}
