use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{kinds::Kind, language::Language, terms::TyLambda};

impl<Lang> Parse for TyLambda<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::ty_lambda_kinded_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["TyLambda Variable", "TyLambda Kind", "TyLambda Term"],
        )?;
        let var = inner.remove(0).as_str().trim();
        let kind_rule = inner.remove(0);
        let kind = Kind::from_pair(kind_rule, ())?;
        let term_rule = inner.remove(0);
        let term = Lang::Term::from_pair(term_rule, ())?;
        Ok(Self::new(var, kind, term))
    }
}
