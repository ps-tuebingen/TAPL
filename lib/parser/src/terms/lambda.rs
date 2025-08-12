use crate::{Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{
    terms::{Lambda, Term},
    types::Type,
};

impl<T, Ty> Parse for Lambda<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::lambda_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Lambda<T, Ty>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Body"])?;
        let var = inner.remove(0).as_str().trim();
        let annot_rule = inner.remove(0);
        let annot = Ty::from_pair(annot_rule, ())?;
        let body_rule = inner.remove(0);
        let body = T::from_pair(body_rule, ())?;
        Ok(Lambda::new(var, annot, body))
    }
}
