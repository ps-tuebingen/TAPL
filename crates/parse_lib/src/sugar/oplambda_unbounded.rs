use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    types::{OpLambda, OpLambdaSub, Top, Type},
};

pub struct OpLambdaUnbounded<Ty>
where
    Ty: Type,
{
    var: String,
    body: Ty,
}

impl<Ty> OpLambdaUnbounded<Ty>
where
    Ty: Type,
{
    pub fn to_oplambda_kinded(self) -> OpLambda<Ty> {
        self.into()
    }

    pub fn to_oplambda_sub(self) -> OpLambdaSub<Ty>
    where
        Top<Ty>: Into<Ty>,
    {
        self.into()
    }
}

impl<Ty> Parse for OpLambdaUnbounded<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::op_lambda_star_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
        let var = inner.remove(0).as_str().trim().to_owned();
        let ty_rule = inner.remove(0);
        let body = Ty::from_pair(ty_rule, ())?;
        Ok(OpLambdaUnbounded { var, body })
    }
}

impl<Ty> From<OpLambdaUnbounded<Ty>> for OpLambda<Ty>
where
    Ty: Type,
{
    fn from(ou: OpLambdaUnbounded<Ty>) -> OpLambda<Ty> {
        OpLambda::new(&ou.var, Kind::Star, ou.body)
    }
}

impl<Ty> From<OpLambdaUnbounded<Ty>> for OpLambdaSub<Ty>
where
    Ty: Type,
    Top<Ty>: Into<Ty>,
{
    fn from(ou: OpLambdaUnbounded<Ty>) -> OpLambdaSub<Ty> {
        OpLambdaSub::new(&ou.var, Top::new_star(), ou.body)
    }
}
