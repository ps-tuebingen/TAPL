use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::IsZero};

impl<Lang> GrammarRuleDescribe for IsZero<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Keyword::IsZero.into(), Symbol::paren(vec![Symbol::Term])].into(),
            "IsZero",
        )
    }
}
