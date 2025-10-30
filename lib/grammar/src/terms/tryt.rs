use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Try};

impl<Lang> GrammarRuleDescribe for Try<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Keyword::Try.into(), Symbol::brack(Symbol::Term)].into(),
            "Variant Case",
        )
    }
}
