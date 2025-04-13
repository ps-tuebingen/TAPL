use super::Term;

#[derive(Clone, Debug)]
pub struct App<T>
where
    T: Term,
{
    pub fun: Box<T>,
    pub arg: Box<T>,
}

impl<T> Term for App<T> where T: Term {}
