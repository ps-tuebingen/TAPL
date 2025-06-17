use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    terms::{LambdaSub, Term},
    types::{Top, Type},
};

pub struct LambdaSubUnbounded<T>
where
    T: Term,
{
    var: String,
    body: T,
}

impl<T> LambdaSubUnbounded<T>
where
    T: Term,
{
    pub fn to_lambda_sub<Ty>(self) -> LambdaSub<T, Ty>
    where
        Ty: Type,
        Top<Ty>: Into<Ty>,
    {
        self.into()
    }
}

impl<T> Parse for LambdaSubUnbounded<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::ty_lambda_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
        let var = inner.remove(0).as_str().trim().to_owned();
        let body_rule = inner.remove(0);
        let body = T::from_pair(body_rule, ())?;
        Ok(LambdaSubUnbounded { var, body })
    }
}

impl<T, Ty> From<LambdaSubUnbounded<T>> for LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
    Top<Ty>: Into<Ty>,
{
    fn from(ls: LambdaSubUnbounded<T>) -> LambdaSub<T, Ty> {
        LambdaSub::new_unbounded(&ls.var, ls.body)
    }
}
