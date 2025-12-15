use crate::{GroupParse, Parse, Rule, pair_span, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{kinds::Kind, language::Language, span::Span, terms::TyLambda};

pub struct TyLambdaStar<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    var: String,
    term: Lang::Term,
    span: Span,
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

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
        let var = inner.remove(0).as_str().trim().to_owned();
        let term_rule = inner.remove(0);
        let term = Lang::Term::from_pair(term_rule, ())?;
        Ok(Self { var, term, span })
    }
}

impl<Lang> From<TyLambdaStar<Lang>> for TyLambda<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn from(ts: TyLambdaStar<Lang>) -> Self {
        Self::new(&ts.var, Kind::Star, ts.term, ts.span)
    }
}
