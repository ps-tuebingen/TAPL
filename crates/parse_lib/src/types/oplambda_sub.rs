use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::types::{OpLambdaSub, Type};

impl<Ty> Parse for OpLambdaSub<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::op_lambda_type;

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<OpLambdaSub<Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Op Lambda Var", "Op Lambda Annot", "Op Lambda Body"],
        )?;
        let var = inner.remove(0).as_str().trim();
        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule, ())?;
        let body_rule = inner.remove(0);
        let body = Ty::from_pair(body_rule, ())?;
        Ok(OpLambdaSub::new(var, ty, body))
    }
}
