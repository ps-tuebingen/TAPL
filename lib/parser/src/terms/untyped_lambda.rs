use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::UntypedLambda};

impl<Lang> Parse for UntypedLambda<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::untyped_lambda_term;

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<UntypedLambda<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Body"])?;
        let var = inner.remove(0).as_str().trim();
        let body_rule = inner.remove(0);
        let body = Lang::Term::from_pair(body_rule, ())?;
        Ok(UntypedLambda::new(var, body))
    }
}
