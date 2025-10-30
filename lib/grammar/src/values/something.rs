use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Something};

impl<Lang> GrammarRuleDescribe for Something<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Keyword::Something.into(), Symbol::paren(Symbol::Value)].into(),
            "Something",
        )
    }
}
