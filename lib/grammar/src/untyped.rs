use crate::{Grammar, GrammarDescribe, Rule, RuleDescribe, symbols::SpecialChar};
use syntax::untyped::Untyped;

impl GrammarDescribe for Untyped {
    fn grammar() -> Grammar {
        Grammar::ty(vec![])
    }
}

impl RuleDescribe for Untyped {
    fn rule() -> Rule {
        Rule::new(SpecialChar::Empty.into(), "Untyped")
    }
}
