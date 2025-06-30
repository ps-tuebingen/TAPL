use crate::{non_terminal::NonTerminal, terminal::Terminal};

pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

impl From<Terminal> for Symbol {
    fn from(t: Terminal) -> Symbol {
        Symbol::Terminal(t)
    }
}

impl From<NonTerminal> for Symbol {
    fn from(nt: NonTerminal) -> Symbol {
        Symbol::NonTerminal(nt)
    }
}
