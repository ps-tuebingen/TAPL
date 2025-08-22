use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{terms::LambdaSub, types::Top};

pub struct LambdaSubStar<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    var: String,
    body: Lang::Term,
}

impl<Lang> LambdaSubStar<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn to_lambda_sub(self) -> LambdaSub<Lang>
    where
        Top<Lang>: Into<Lang::Type>,
    {
        self.into()
    }
}

impl<Lang> Parse for LambdaSubStar<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::ty_lambda_star_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
        let var = inner.remove(0).as_str().trim().to_owned();
        let body_rule = inner.remove(0);
        let body = Lang::Term::from_pair(body_rule, ())?;
        Ok(LambdaSubStar { var, body })
    }
}

impl<Lang> From<LambdaSubStar<Lang>> for LambdaSub<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
    Top<Lang>: Into<Lang::Type>,
{
    fn from(ls: LambdaSubStar<Lang>) -> LambdaSub<Lang> {
        LambdaSub::new_unbounded(&ls.var, ls.body)
    }
}
