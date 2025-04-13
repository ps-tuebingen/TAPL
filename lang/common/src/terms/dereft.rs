use super::Term;
pub struct Deref<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Deref<T> where T: Term {}
