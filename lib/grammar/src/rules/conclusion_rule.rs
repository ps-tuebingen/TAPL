use crate::symbols::{SpecialChar, Symbol};
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

impl ConclusionRule {
    pub fn new<S1, S2, S3, S4>(env: S1, input: S2, separator: S3, output: S4) -> ConclusionRule
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
        S3: Into<Symbol>,
        S4: Into<Symbol>,
    {
        ConclusionRule {
            env: env.into(),
            input: input.into(),
            separator: separator.into(),
            output: output.into(),
        }
    }

    /// Create a typing conclusion with input and output
    /// Gamma |-> input : output
    pub fn typing<S1, S2>(input: S1, output: S2) -> ConclusionRule
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        ConclusionRule::new(SpecialChar::Gamma, input, SpecialChar::Colon, output)
    }

    /// Create a subtyping conclusion with input and output
    /// Gamma |-> input <: output
    pub fn subtyping<S1, S2>(input: S1, output: S2) -> ConclusionRule
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        ConclusionRule::new(SpecialChar::Gamma, input, SpecialChar::LessColon, output)
    }

    ///Create kinding conclusion with input and output
    ///Gamma |-> input :: output
    pub fn kinding<S1, S2>(input: S1, output: S2) -> ConclusionRule
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        ConclusionRule::new(SpecialChar::Gamma, input, SpecialChar::DoubleColon, output)
    }

    pub fn eval<S1, S2>(input: S1, output: S2) -> ConclusionRule
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        ConclusionRule::new(SpecialChar::Gamma, input, SpecialChar::Mapsto, output)
    }

    /// Change the environment of `self`
    /// env |-> self.input self.separator self.output
    pub fn with_env<S>(self, env: S) -> ConclusionRule
    where
        S: Into<Symbol>,
    {
        ConclusionRule {
            env: env.into(),
            input: self.input,
            separator: self.separator,
            output: self.output,
        }
    }

    /// conclusion for input in Gamma
    pub fn lookup_env<S>(input: S) -> ConclusionRule
    where
        S: Into<Symbol>,
    {
        ConclusionRule::new(
            SpecialChar::Empty,
            input,
            SpecialChar::Element,
            SpecialChar::Gamma,
        )
    }
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
