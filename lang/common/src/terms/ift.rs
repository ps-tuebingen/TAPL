use super::Term;

pub struct If<T> where T:Term{
    if_cond:Box<T>,
    them_term:Box<T>,
    else_term:Box<T>
}

impl<T> Term for If<T> where T:Term{}
