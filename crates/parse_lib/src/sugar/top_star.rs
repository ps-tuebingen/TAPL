use crate::{Parse, Rule, errors::ParserError};
use pest::iterators::Pair;
use std::marker::PhantomData;
use syntax::types::{Top, Type};

pub struct TopStar<Ty>
where
    Ty: Type,
{
    phantom: PhantomData<Ty>,
}

impl<Ty> TopStar<Ty>
where
    Ty: Type,
{
    pub fn to_top(self) -> Top<Ty>
where {
        self.into()
    }
}

impl<Ty> Parse for TopStar<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::top_type_star;

    fn from_pair(_: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<TopStar<Ty>, ParserError> {
        Ok(TopStar {
            phantom: PhantomData,
        })
    }
}

impl<Ty> From<TopStar<Ty>> for Top<Ty>
where
    Ty: Type,
{
    fn from(_: TopStar<Ty>) -> Top<Ty> {
        Top::new_star()
    }
}
