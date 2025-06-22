use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::types::{Fun, Type};

impl<Ty> Parse for Fun<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = Ty;

    const RULE: Rule = Rule::fun_type;
    fn from_pair(p: Pair<'_, Rule>, ty: Self::LeftRecArg) -> Result<Fun<Ty>, Self::ParseError> {
        let to_rule = pair_to_n_inner(p, vec!["Function Return Type"])?.remove(0);
        let to_ty = Ty::from_pair(to_rule, ())?;
        Ok(Fun::new(ty, to_ty))
    }
}
