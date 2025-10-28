use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Exception};

impl<Lang> GrammarRuleDescribe for Exception<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Err, Some(Symbol::Type), vec![Symbol::Value]),
            "Exception",
        )
    }
}
