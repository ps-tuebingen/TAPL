use super::{UntypedArithmetic, terms::Term};
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    terms::{False, If, IsZero, Num, Pred, Succ, True},
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = UntypedArithmetic;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::Num(num) => num.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::If(ift) => ift.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(True::<UntypedArithmetic>::rules());
        rules.extend(False::<UntypedArithmetic>::rules());
        rules.extend(If::<UntypedArithmetic>::rules());
        rules.extend(Num::<UntypedArithmetic>::rules());
        rules.extend(Succ::<UntypedArithmetic>::rules());
        rules.extend(Pred::<UntypedArithmetic>::rules());
        rules.extend(IsZero::<UntypedArithmetic>::rules());
        rules
    }
}
