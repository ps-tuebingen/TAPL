use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    terms::{App, Lambda, Term},
    types::{Type, Unit},
};

pub struct Sequence<T>
where
    T: Term,
{
    fst: T,
    snd: T,
}

impl<T> Sequence<T>
where
    T: Term,
{
    pub fn to_term<Ty>(self) -> T
    where
        App<T>: Into<T>,
        Ty: Type,
        Lambda<T, Ty>: Into<T>,
        Unit<Ty>: Into<Ty>,
    {
        App::new(Lambda::new("_", Unit::new(), self.snd), self.fst).into()
    }
}

impl<T> Parse for Sequence<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = T;

    const RULE: Rule = Rule::sequence;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Sequence<T>, ParserError> {
        let term_rule = pair_to_n_inner(p, vec!["Sequence Second Term"])?.remove(0);
        let term = T::from_pair(term_rule, ())?;
        Ok(Sequence { fst: t, snd: term })
    }
}
