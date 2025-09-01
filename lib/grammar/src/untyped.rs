use crate::{Grammar, GrammarDescribe, Rule, RuleDescribe, symbols::SpecialChar};
use syntax::{language::Language, untyped::Untyped};

impl<Lang> GrammarDescribe for Untyped<Lang>
where
    Lang: Language,
{
    fn grammar() -> Grammar {
        Grammar::ty(vec![])
    }
}

impl<Lang> RuleDescribe for Untyped<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Empty.into(), "Untyped")
    }
}
