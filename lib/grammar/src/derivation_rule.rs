use crate::Symbol;

/// Rule for a typing derivation
/// For example
/// Gamma |-> t1:ty2->ty1   Gamma |-> t2:ty2
/// ---------------------------------------
///          Gamma |-> t1 t2: ty2
#[derive(Debug)]
pub struct DerivationRule {
    /// premises of the derivation rule
    pub premises: Vec<ConclusionRule>,
    /// Name of the rule
    label: String,
    /// Conclusion of the rule
    conclusion: ConclusionRule,
}

/// Conclusion for a Derivation rule
/// for example Gamma |-> t:ty
#[derive(Debug)]
pub struct ConclusionRule {
    /// The symbol used for the environment
    /// usually Gamma
    env: Symbol,
    /// The symbol used in the left hand side of the rule (e.g. `t` for a term)
    input: Symbol,
    /// The separator symbol between input and output (e.g. `:` for a term)
    separator: Symbol,
    /// The symbol unsed in the right hand side of the rule (e.g. `ty` for a type)
    output: Symbol,
}
