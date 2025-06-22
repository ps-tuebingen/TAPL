use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    types::{Forall, Type},
};

impl<Ty> Parse for Forall<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::forall_unbounded_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Forall<Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Kind", "Forall Body"])?;
        let var = pair_to_n_inner(inner.remove(0), vec!["Forall Variable"])?
            .remove(0)
            .as_str()
            .trim();
        let kind_rule = inner.remove(0);
        let kind = Kind::from_pair(kind_rule, ())?;
        let body_rule = inner.remove(0);
        let body = Ty::from_pair(body_rule, ())?;
        Ok(Forall::new(var, kind, body))
    }
}
