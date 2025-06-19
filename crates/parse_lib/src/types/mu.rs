use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::types::{Mu, Type};

impl<Ty> Parse for Mu<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::mu_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Mu<Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Mu Variable", "Mu Body"])?;
        let var = inner.remove(0).as_str().trim();
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        Ok(Mu::new(var, ty))
    }
}
