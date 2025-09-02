use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{
    language::Language,
    {kinds::Kind, types::OpLambda},
};

impl<Lang> Parse for OpLambda<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::op_lambda_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<OpLambda<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Op Lambda Var", "Op Lambda Annot", "Op Lambda Body"],
        )?;
        let var = inner.remove(0).as_str().trim();
        let kind_rule = inner.remove(0);
        let kind = Kind::from_pair(kind_rule, ())?;
        let body_rule = inner.remove(0);
        let body = Lang::Type::from_pair(body_rule, ())?;
        Ok(OpLambda::new(var, kind, body))
    }
}
