use syntax::{env::Environment, terms::Term, types::Type};

pub struct Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    env: Environment<Ty>,
    term: T,
    ty: Ty,
    premises: Vec<Derivation<T, Ty>>,
}
