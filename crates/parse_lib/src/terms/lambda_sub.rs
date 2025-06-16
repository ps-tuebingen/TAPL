use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    terms::{LambdaSub, Term},
    types::Type,
};

impl<T, Ty> Parse for LambdaSub<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::lambda_sub_term;

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<LambdaSub<T, Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Type Variable", "Super Type", "Type Abstraction Body"],
        )?;
        let var = inner.remove(0).as_str().trim();
        let super_rule = inner.remove(0);
        let sup_ty = Ty::from_pair(super_rule, ())?;
        let body_rule = inner.remove(0);
        let body = T::from_pair(body_rule, ())?;
        Ok(LambdaSub::new(var, sup_ty, body))
    }
}
