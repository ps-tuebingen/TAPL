use super::Term;

pub struct Pair<T>
where
    T: Term,
{
    fst: Box<T>,
    snd: Box<T>,
}

impl<T> Term for Pair<T> where T: Term {}
