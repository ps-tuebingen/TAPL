use super::Term;

pub struct Tuple<T>
where
    T: Term,
{
    terms: Vec<T>,
}

impl<T> Term for Tuple<T> where T: Term {}
