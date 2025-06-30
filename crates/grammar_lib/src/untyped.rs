use crate::{Grammar, GrammarDescribe, Rule, RuleDescribe, Symbol};
use syntax::untyped::Untyped;

impl GrammarDescribe for Untyped {
    fn grammar() -> Grammar {
        Grammar::ty(vec![])
    }
}

impl RuleDescribe for Untyped {
    fn rule() -> Rule {
        Rule::new(Symbol::term(""), "Untyped")
    }
}
