use std::fmt;
use syntax::language::Language;

pub mod rules;
pub mod step;

pub use step::EvalStep;

#[derive(Debug)]
pub struct EvalTrace<Lang>
where
    Lang: Language,
{
    pub steps: Vec<EvalStep<Lang>>,
    val: Lang::Value,
}

impl<Lang> EvalTrace<Lang>
where
    Lang: Language,
{
    pub fn new<V1>(steps: Vec<EvalStep<Lang>>, val: V1) -> EvalTrace<Lang>
    where
        V1: Into<Lang::Value>,
    {
        EvalTrace {
            steps,
            val: val.into(),
        }
    }

    pub fn val(&self) -> Lang::Value {
        self.val.clone()
    }

    pub fn congruence(self, into_fun: &impl Fn(Lang::Term) -> Lang::Term) -> Vec<EvalStep<Lang>> {
        self.steps
            .into_iter()
            .map(|step| step.congruence(into_fun))
            .collect()
    }
}

impl<Lang> fmt::Display for EvalTrace<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for step in self.steps.iter() {
            writeln!(f, "{step}")?;
        }
        writeln!(f)?;
        write!(f, "{}", self.val)
    }
}
