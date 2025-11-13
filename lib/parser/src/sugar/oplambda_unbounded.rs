use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    language::Language,
    types::{OpLambda, OpLambdaSub, Top},
};

pub struct OpLambdaUnbounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    var: String,
    body: Lang::Type,
}

impl<Lang> OpLambdaUnbounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn to_oplambda_kinded(self) -> OpLambda<Lang> {
        self.into()
    }

    pub fn to_oplambda_sub(self) -> OpLambdaSub<Lang>
    where
        Top<Lang>: Into<Lang::Type>,
    {
        self.into()
    }
}

impl<Lang> Parse for OpLambdaUnbounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::op_lambda_star_type;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
        let var = inner.remove(0).as_str().trim().to_owned();
        let ty_rule = inner.remove(0);
        let body = Lang::Type::from_pair(ty_rule, ())?;
        Ok(Self { var, body })
    }
}

impl<Lang> From<OpLambdaUnbounded<Lang>> for OpLambda<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn from(ou: OpLambdaUnbounded<Lang>) -> Self {
        Self::new(&ou.var, Kind::Star, ou.body)
    }
}

impl<Lang> From<OpLambdaUnbounded<Lang>> for OpLambdaSub<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
    Top<Lang>: Into<Lang::Type>,
{
    fn from(ou: OpLambdaUnbounded<Lang>) -> Self {
        Self::new(&ou.var, Top::new_star(), ou.body)
    }
}
