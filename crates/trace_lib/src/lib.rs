use syntax::{terms::Term, values::Value};

pub struct EvalStep<T>
where
    T: Term,
{
    source: T,
    target: T,
}

pub struct EvalTrace<T, V>
where
    T: Term,
    V: Value,
{
    start: T,
    steps: Vec<EvalStep<T>>,
    end: V,
}
