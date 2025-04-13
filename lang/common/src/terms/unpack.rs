use super::Term;
use crate::{TypeVar, Var};

pub struct Unpack<T>
where
    T: Term,
{
    ty_name: TypeVar,
    term_name: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Term for Unpack<T> where T: Term {}
