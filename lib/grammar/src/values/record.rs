use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, values::Record};

impl<Lang> GrammarRuleDescribe for Record<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            Symbol::brack(Symbol::many(vec![
                Symbol::Label,
                SpecialChar::Equals.into(),
                Symbol::Value,
            ])),
            "Record",
        )
    }
}
