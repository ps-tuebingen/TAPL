use syntax::{terms::Term, values::Value};

pub mod errors;
pub mod rules;
pub mod step;

pub use step::EvalStep;

pub struct EvalTrace<T, V>
where
    T: Term,
    V: Value,
{
    steps: Vec<EvalStep<T>>,
    val: V,
}

impl<T, V> EvalTrace<T, V>
where
    T: Term,
    V: Value,
{
    pub fn val(&self) -> V {
        self.val.clone()
    }

    pub fn congruence(self, into_fun: &impl Fn(T) -> T) -> Vec<EvalStep<T>> {
        self.steps
            .into_iter()
            .map(|step| step.congruence(into_fun))
            .collect()
    }
}
