use super::Term;

pub struct TryWithVal<T>
where
    T: Term,
{
    term: Box<T>,
    handler: Box<T>,
}

impl<T> Term for TryWithVal<T> where T: Term {}
