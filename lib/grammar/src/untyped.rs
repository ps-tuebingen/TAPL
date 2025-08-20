use crate::{Grammar, GrammarDescribe, Rule, RuleDescribe, symbols::SpecialChar};
use syntax::{terms::Term, untyped::Untyped};

impl<T> GrammarDescribe for Untyped<T>
where
    T: Term,
{
    fn grammar() -> Grammar {
        Grammar::ty(vec![])
    }
}

impl<T> RuleDescribe for Untyped<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Empty.into(), "Untyped")
    }
}
