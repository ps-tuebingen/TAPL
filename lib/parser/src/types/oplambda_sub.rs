use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, types::OpLambdaSub};

impl<Lang> Parse for OpLambdaSub<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::op_lambda_type;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Op Lambda Var", "Op Lambda Annot", "Op Lambda Body"],
        )?;
        let var = inner.remove(0).as_str().trim();
        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;
        let body_rule = inner.remove(0);
        let body = Lang::Type::from_pair(body_rule, ())?;
        Ok(Self::new(var, ty, body))
    }
}
