use super::Term;

pub struct Fst<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Fst<T> where T: Term {}
