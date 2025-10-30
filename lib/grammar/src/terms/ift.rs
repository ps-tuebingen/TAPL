use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::If};

impl<Lang> GrammarRuleDescribe for If<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
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
