use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Exception};

impl<Lang> RuleDescribe for Exception<Lang>
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
