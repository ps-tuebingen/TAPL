use crate::{errors::ParserError, pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    types::{OpLambda, Type},
};

impl<Ty> Parse for OpLambda<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::op_lambda_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<OpLambda<Ty>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Op Lambda Var", "Op Lambda Annot", "Op Lambda Body"],
        )?;
        let var = inner.remove(0).as_str().trim();
        let kind_rule = inner.remove(0);
        let kind = Kind::from_pair(kind_rule, ())?;
        let body_rule = inner.remove(0);
        let body = Ty::from_pair(body_rule, ())?;
        Ok(OpLambda::new(var, kind, body))
    }
}
