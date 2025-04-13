use super::Term;
use crate::{types::Type, Var};

pub struct ListCase<T, Ty>
where
    T: Term,
    Ty: Type,
{
    bound_term: Box<T>,
    list_ty: Ty,
    nil_rhs: Box<T>,
    cons_fst: Var,
    cons_rst: Var,
    cons_rhs: Box<T>,
}

impl<T, Ty> Term for ListCase<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
