use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::ForallBounded};

impl<Lang> GrammarRuleDescribe for ForallBounded<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Forall.into(),
                Symbol::less_colon_sep(Symbol::Typevariable, Symbol::Type),
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into(),
            "Universal Type",
        )
    }
}
