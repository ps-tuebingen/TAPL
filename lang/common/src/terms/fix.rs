use super::Term;

pub struct Fix<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Fix<T> where T: Term {}
