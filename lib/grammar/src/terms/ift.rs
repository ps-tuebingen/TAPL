use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::If};

impl<Lang> GrammarRuleDescribe for If<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                Keyword::If.into(),
                Symbol::brack(Symbol::Term),
                Keyword::Else.into(),
                Symbol::brack(Symbol::Term),
            ]
            .into(),
            "If",
        )
    }
}
