use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct If<T>
where
    T: Term,
{
    if_cond: Box<T>,
    then_term: Box<T>,
    else_term: Box<T>,
}

impl<T> Term for If<T> where T: Term {}

impl<T> fmt::Display for If<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.if_cond, self.then_term, self.else_term
        )
    }
}
