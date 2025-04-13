use super::Term;

pub struct Ref<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Ref<T> where T: Term {}
