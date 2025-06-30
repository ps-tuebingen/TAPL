mod grammar;
mod rule;
mod symbol;

mod terms;
mod untyped;
mod values;

pub use grammar::{Grammar, LanguageGrammar};
pub use rule::Rule;
pub use symbol::Symbol;

pub trait GrammarDescribe {
    fn grammar() -> Grammar;
}

pub trait LanguageDescribe {
    fn grammars() -> LanguageGrammar;
}

pub trait RuleDescribe {
    fn rule() -> Rule;
}
