use super::Term;

pub struct Assign<T>
where
    T: Term,
{
    lhs: Box<T>,
    rhs: Box<T>,
}

impl<T> Term for Assign<T> where T: Term {}
