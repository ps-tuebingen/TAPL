use crate::{Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::types::{List, Type};

impl<Ty> Parse for List<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::list_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<List<Ty>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["List Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        Ok(List::new(ty))
    }
}
