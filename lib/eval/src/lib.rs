use std::{collections::HashSet, rc::Rc};

pub mod terms;

use errors::eval_error::EvalError;
use grammar::DerivationRule;
use syntax::{eval_context::EvalContext, language::Language, program::Program};
use trace::EvalTrace;

/// Trait for evaluating terms
pub trait Eval: Sized {
    /// The language terms belong to
    type Lang: Language;

    /// Evaluate `self` with empty environment
    /// # Errors
    /// Returns an error if evaluation gets stuck
    fn eval_start(self) -> Result<EvalTrace<Self::Lang>, EvalError> {
        self.eval(&mut EvalContext::default())
    }

    /// Evaluat `self` with a given environment
    /// # Errors
    /// Returns an Error if evaluation gets stuck
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError>;

    /// Evaluation rules for `Self`
    /// usually one or more congruence rules and one evaluation rule
    fn rules() -> HashSet<DerivationRule>;
}

impl<T> Eval for Rc<T>
where
    T: Eval + Clone,
{
    type Lang = T::Lang;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        Self::unwrap_or_clone(self).eval(env)
    }

    fn rules() -> HashSet<DerivationRule> {
        T::rules()
    }
}

/// Evaluat the `main` definition in a [`Program`]
/// # Errors
/// returns an error if evaluating the body gets stuck
pub fn eval_main<Lang>(prog: Program<Lang>) -> Result<EvalTrace<Lang>, EvalError>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
{
    let mut ctx = EvalContext::<Lang>::from_prog(&prog);
    prog.main.eval(&mut ctx)
}
