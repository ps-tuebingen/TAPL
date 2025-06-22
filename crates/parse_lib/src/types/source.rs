use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::types::{Source, Type};

impl<Ty> Parse for Source<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();
    const RULE: Rule = Rule::source_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Source<Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Source Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        Ok(Source::new(ty))
    }
}
