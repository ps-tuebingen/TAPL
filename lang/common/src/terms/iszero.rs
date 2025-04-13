use super::Term;

pub struct IsZero<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for IsZero<T> where T: Term {}
