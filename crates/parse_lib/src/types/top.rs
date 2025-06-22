use crate::{errors::ParserError, pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    types::{Top, Type},
};

impl<Ty> Parse for Top<Ty>
where
    Ty: Type + Parse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::top_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let kind_rule = pair_to_n_inner(p, vec!["Kind"])?.remove(0);
        let kind = Kind::from_pair(kind_rule, ())?;
        Ok(Top::new(kind))
    }
}
