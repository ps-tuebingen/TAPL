use crate::{Grammar, GrammarDescribe, GrammarRule, GrammarRuleDescribe, symbols::SpecialChar};
use syntax::{language::Language, untyped::Untyped};

impl<Lang> GrammarDescribe for Untyped<Lang>
where
    Lang: Language,
{
    fn grammar() -> Grammar {
        Grammar::ty(vec![])
    }
}

impl<Lang> GrammarRuleDescribe for Untyped<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(SpecialChar::Empty.into(), "Untyped")
    }
}
