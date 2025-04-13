use super::Term;

pub struct Something<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Something<T> where T: Term {}
