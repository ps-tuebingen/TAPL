use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Lambda};

impl<Lang> Parse for Lambda<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::lambda_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Lambda<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Body"])?;
        let var = inner.remove(0).as_str().trim();
        let annot_rule = inner.remove(0);
        let annot = Lang::Type::from_pair(annot_rule, ())?;
        let body_rule = inner.remove(0);
        let body = Lang::Term::from_pair(body_rule, ())?;
        Ok(Lambda::new(var, annot, body))
    }
}
