use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    terms::{Term, TyLambda},
};

impl<T> Parse for TyLambda<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::ty_lambda_kinded_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<TyLambda<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["TyLambda Variable", "TyLambda Kind", "TyLambda Term"],
        )?;
        let var = inner.remove(0).as_str().trim();
        let kind_rule = inner.remove(0);
        let kind = Kind::from_pair(kind_rule, ())?;
        let term_rule = inner.remove(0);
        let term = T::from_pair(term_rule, ())?;
        Ok(TyLambda::new(var, kind, term))
    }
}
