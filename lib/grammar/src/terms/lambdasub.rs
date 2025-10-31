use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::LambdaSub};

impl<Lang> GrammarRuleDescribe for LambdaSub<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Lambda.into(),
                Symbol::less_colon_sep(Symbol::Variable, Symbol::Type),
                SpecialChar::Dot.into(),
                Symbol::Term,
            ]
            .into(),
            "Lambda Sub",
        )
    }
}
