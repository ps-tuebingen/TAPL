use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{kinds::Kind, language::Language, terms::TyLambda};

pub struct TyLambdaStar<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    var: String,
    term: Lang::Term,
}

impl<Lang> TyLambdaStar<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn to_tylambda(self) -> TyLambda<Lang> {
        self.into()
    }
}

impl<Lang> Parse for TyLambdaStar<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::ty_lambda_star_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
        let var = inner.remove(0).as_str().trim().to_owned();
        let term_rule = inner.remove(0);
        let term = Lang::Term::from_pair(term_rule, ())?;
        Ok(TyLambdaStar { var, term })
    }
}

impl<Lang> From<TyLambdaStar<Lang>> for TyLambda<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn from(ts: TyLambdaStar<Lang>) -> TyLambda<Lang> {
        TyLambda::new(&ts.var, Kind::Star, ts.term)
    }
}
