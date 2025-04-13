use super::Term;

pub struct Succ<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Succ<T> where T: Term {}
