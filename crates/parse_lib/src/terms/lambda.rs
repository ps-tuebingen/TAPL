use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    terms::{Lambda, Term},
    types::Type,
};

impl<T, Ty> Parse for Lambda<T, Ty>
where
    T: Term + Parse,
    Ty: Type + Parse,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    fn rule() -> Rule {
        Rule::lambda_term
    }

    fn from_pair(p: Pair<'_, Rule>) -> Result<Lambda<T, Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Body"])?;
        let var = inner.remove(0).as_str().trim();
        let annot_rule = inner.remove(0);
        let annot = Ty::from_pair(annot_rule)?;
        let body_rule = inner.remove(0);
        let body = T::from_pair(body_rule)?;
        Ok(Lambda::new(var, annot, body))
    }
}
