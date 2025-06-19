use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::types::{Sink, Type};

impl<Ty> Parse for Sink<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::sink_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Sink<Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Sink Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        Ok(Sink::new(ty))
    }
}
