use crate::{non_terminal::NonTerminal, rule::Rule};

pub struct Grammar {
    symbol: NonTerminal,
    description: String,
    alternatives: Vec<Rule>,
}
