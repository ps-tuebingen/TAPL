use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Cast};

impl<Lang> GrammarRuleDescribe for Cast<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Term, Keyword::As.into(), Symbol::Term].into(),
            "Cast",
        )
    }
}
