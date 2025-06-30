mod grammar;
mod non_terminal;
mod rule;
mod symbol;
mod terminal;

pub use grammar::Grammar;
pub use rule::Rule;

pub trait GrammarDescribe: RuleDescribe {
    fn describe(&self) -> Grammar;
}

pub trait RuleDescribe {
    fn rule(&self) -> Rule;
}
