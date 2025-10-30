use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Right};

impl<Lang> GrammarRuleDescribe for Right<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Right.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(Symbol::Value),
            ]
            .into(),
            "Right Injection",
        )
    }
}
