use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    terms::{Term, TyLambda},
};

pub struct TyLambdaStar<T>
where
    T: Term,
{
    var: String,
    term: T,
}

impl<T> TyLambdaStar<T>
where
    T: Term,
{
    pub fn to_tylambda(self) -> TyLambda<T> {
        self.into()
    }
}

impl<T> Parse for TyLambdaStar<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::ty_lambda_star_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
        let var = inner.remove(0).as_str().trim().to_owned();
        let term_rule = inner.remove(0);
        let term = T::from_pair(term_rule, ())?;
        Ok(TyLambdaStar { var, term })
    }
}

impl<T> From<TyLambdaStar<T>> for TyLambda<T>
where
    T: Term,
{
    fn from(ts: TyLambdaStar<T>) -> TyLambda<T> {
        TyLambda::new(&ts.var, Kind::Star, ts.term)
    }
}
