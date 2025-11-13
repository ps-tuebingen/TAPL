use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    language::Language,
    types::{Exists, ExistsBounded, Top},
};

pub struct ExistsUnbounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    var: String,
    body_ty: Lang::Type,
}

impl<Lang> ExistsUnbounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn to_exists_bounded(self) -> ExistsBounded<Lang>
    where
        Lang: Language,
        Lang::Term: GroupParse,
        Lang::Type: GroupParse,
        Top<Lang>: Into<Lang::Type>,
    {
        self.into()
    }

    pub fn to_exists_kinded(self) -> Exists<Lang> {
        self.into()
    }
}

impl<Lang> Parse for ExistsUnbounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::exists_unbounded_type;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Exists Variable", "Exists Type"])?;
        let var_rule = inner.remove(0);
        let mut var_inner = pair_to_n_inner(var_rule, vec!["Exists Variable"])?;
        let var = var_inner.remove(0).as_str().trim().to_owned();
        let body_rule = inner.remove(0);
        let body_ty = Lang::Type::from_pair(body_rule, ())?;

        Ok(Self { var, body_ty })
    }
}

impl<Lang> From<ExistsUnbounded<Lang>> for ExistsBounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
    Top<Lang>: Into<Lang::Type>,
{
    fn from(eu: ExistsUnbounded<Lang>) -> Self {
        Self::new_unbounded(&eu.var, Kind::Star, eu.body_ty)
    }
}

impl<Lang> From<ExistsUnbounded<Lang>> for Exists<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn from(eu: ExistsUnbounded<Lang>) -> Self {
        Self::new(&eu.var, Kind::Star, eu.body_ty)
    }
}
