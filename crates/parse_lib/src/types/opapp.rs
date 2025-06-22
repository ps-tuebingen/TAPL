use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::types::{OpApp, Type};

impl<Ty> Parse for OpApp<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = Ty;

    const RULE: Rule = Rule::op_app_type;

    fn from_pair(p: Pair<'_, Rule>, ty: Self::LeftRecArg) -> Result<OpApp<Ty>, Self::ParseError> {
        let arg_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
        let arg = Ty::from_pair(arg_rule, ())?;
        Ok(OpApp::new(ty, arg))
    }
}
