use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Tuple<T>
where
    T: Term,
{
    terms: Vec<T>,
}

impl<T> Term for Tuple<T> where T: Term {}

impl<T> fmt::Display for Tuple<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self.terms.iter().map(|t| t.to_string()).collect();
        ts.sort();
        write!(f, "( {} )", ts.join(", "))
    }
}
