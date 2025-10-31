use crate::symbols::Symbol;
use std::fmt;

/// Conclusion for a Derivation rule
/// for example Gamma |-> t:ty
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConclusionRule {
    /// The symbol used for the environment
    /// usually Gamma
    pub env: Symbol,
    /// The symbol used in the left hand side of the rule (e.g. `t` for a term)
    pub input: Symbol,
    /// The separator symbol between input and output (e.g. `:` for a term)
    pub separator: Symbol,
    /// The symbol unsed in the right hand side of the rule (e.g. `ty` for a type)
    pub output: Symbol,
}

impl fmt::Display for ConclusionRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} |-> {}{}{}",
            self.env, self.input, self.separator, self.output
        )
    }
}
