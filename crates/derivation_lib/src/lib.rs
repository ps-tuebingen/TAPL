use syntax::{terms::Term, types::Type};

pub struct Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    env: String,
    term: T,
    ty: Ty,
    premises: Vec<Derivation<T, Ty>>,
}
