use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Cons};

impl<Lang> GrammarRuleDescribe for Cons<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                Keyword::Cons.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(Symbol::comma_sep(Symbol::Value, Symbol::Value)),
            ]
            .into(),
            "Cons",
        )
    }
}
