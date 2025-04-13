use super::Term;

pub struct Projection<T>
where
    T: Term,
{
    term: Box<T>,
    index: usize,
}

impl<T> Term for Projection<T> where T: Term {}
