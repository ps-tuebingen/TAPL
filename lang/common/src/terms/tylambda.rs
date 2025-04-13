use super::Term;
use crate::Var;

pub struct TyLambda<T>
where
    T: Term,
{
    var: Var,
    term: Box<T>,
}

impl<T> Term for TyLambda<T> where T: Term {}
