use super::Term;

pub struct Try<T>
where
    T: Term,
{
    term: Box<T>,
    handler: Box<T>,
}

impl<T> Term for Try<T> where T: Term {}
