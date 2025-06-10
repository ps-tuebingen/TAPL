use syntax::terms::Term;
use eval::values::Value;

pub struct EvalStep<T>
where
    T: Term,
{
    source: T,
    target: T,
}

pub struct EvalTrace<T,V> where T:Term,V:Value
