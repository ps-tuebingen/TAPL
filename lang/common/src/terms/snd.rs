use super::Term;

pub struct Snd<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Snd<T> where T: Term {}
