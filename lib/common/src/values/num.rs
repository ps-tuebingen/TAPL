use super::Value;
use crate::{language::LanguageTerm, terms::Num as NumT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Num<T>
where
    T: LanguageTerm,
{
    pub num: i64,
    phantom: PhantomData<T>,
}

impl<T> Num<T>
where
    T: LanguageTerm,
{
    pub fn new(i: i64) -> Num<T> {
        Num {
            num: i,
            phantom: PhantomData,
        }
    }
}

impl<T> Value for Num<T>
where
    T: LanguageTerm,
{
    type Term = NumT<T>;
}

impl<T> From<Num<T>> for NumT<T>
where
    T: LanguageTerm,
{
    fn from(n: Num<T>) -> NumT<T> {
        NumT::new(n.num)
    }
}

impl<T> fmt::Display for Num<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
