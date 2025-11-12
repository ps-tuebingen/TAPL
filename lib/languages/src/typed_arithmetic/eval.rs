use super::{TypedArithmetic, terms::Term, types::Type};
use check::Normalize;
use derivations::{Derivation, NormalizingDerivation};
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    eval_context::EvalContext,
    terms::{False, If, IsZero, Num, Pred, Succ, True},
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = TypedArithmetic;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::IsZero(isz) => isz.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(True::<TypedArithmetic>::rules());
        rules.extend(False::<TypedArithmetic>::rules());
        rules.extend(If::<TypedArithmetic>::rules());
        rules.extend(Num::<TypedArithmetic>::rules());
        rules.extend(Succ::<TypedArithmetic>::rules());
        rules.extend(Pred::<TypedArithmetic>::rules());
        rules.extend(IsZero::<TypedArithmetic>::rules());
        rules
    }
}

impl Normalize for Type {
    type Lang = TypedArithmetic;
    fn normalize(self, _: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
