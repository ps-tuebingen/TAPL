use super::rules::EvaluationRule;
use syntax::terms::Term;

pub struct EvalStep<T>
where
    T: Term,
{
    source: T,
    rule: EvaluationRule,
    pub(crate) target: T,
}

impl<T> EvalStep<T>
where
    T: Term,
{
    pub fn congruence(self, into_fun: &impl Fn(T) -> T) -> EvalStep<T> {
        EvalStep {
            source: into_fun(self.source),
            rule: self.rule,
            target: into_fun(self.target),
        }
    }

    pub fn app_abs<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::AppAbs,
            target: target.into(),
        }
    }
}
