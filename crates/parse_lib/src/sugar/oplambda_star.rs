use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    types::{OpLambda, Type},
};

pub struct OpLambdaStar<Ty>
where
    Ty: Type,
{
    var: String,
    body: Ty,
}

impl<Ty> OpLambdaStar<Ty>
where
    Ty: Type,
{
    pub fn to_oplambda(self) -> OpLambda<Ty> {
        self.into()
    }
}

impl<Ty> Parse for OpLambdaStar<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::op_lambda_star_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
        let var = inner.remove(0).as_str().trim().to_owned();
        let ty_rule = inner.remove(0);
        let body = Ty::from_pair(ty_rule, ())?;
        Ok(OpLambdaStar { var, body })
    }
}

impl<Ty> From<OpLambdaStar<Ty>> for OpLambda<Ty>
where
    Ty: Type,
{
    fn from(ts: OpLambdaStar<Ty>) -> OpLambda<Ty> {
        OpLambda::new(&ts.var, Kind::Star, ts.body)
    }
}
