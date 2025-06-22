use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    types::{Top, Type},
};

impl<Ty> Parse for Top<Ty>
where
    Ty: Type + Parse,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::top_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, Self::ParseError> {
        let kind_rule = pair_to_n_inner(p, vec!["Kind"])?.remove(0);
        let kind = Kind::from_pair(kind_rule, ())?;
        Ok(Top::new(kind))
    }
}
