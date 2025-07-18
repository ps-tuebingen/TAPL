use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::types::{Fun, Type};

impl<Ty> Parse for Fun<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = Ty;

    const RULE: Rule = Rule::fun_type;
    fn from_pair(p: Pair<'_, Rule>, ty: Self::LeftRecArg) -> Result<Fun<Ty>, ParserError> {
        let to_rule = pair_to_n_inner(p, vec!["Function Return Type"])?.remove(0);
        let to_ty = Ty::from_pair(to_rule, ())?;
        Ok(Fun::new(ty, to_ty))
    }
}
