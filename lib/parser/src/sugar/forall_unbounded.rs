use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    language::Language,
    types::{Forall, ForallBounded, Top},
};

pub struct ForallUnbounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    var: String,
    body_ty: Lang::Type,
}

impl<Lang> ForallUnbounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn to_forall_bounded(self) -> ForallBounded<Lang>
    where
        Top<Lang>: Into<Lang::Type>,
    {
        self.into()
    }

    pub fn to_forall_kinded(self) -> Forall<Lang> {
        self.into()
    }
}

impl<Lang> Parse for ForallUnbounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::forall_unbounded_type;

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<ForallUnbounded<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Body"])?;
        let var_rule = inner.remove(0);
        let mut var_inner = pair_to_n_inner(var_rule, vec!["Forall Variable"])?;
        let var = var_inner.remove(0).as_str().trim().to_owned();
        let body_rule = inner.remove(0);
        let body_ty = Lang::Type::from_pair(body_rule, ())?;
        Ok(ForallUnbounded { var, body_ty })
    }
}

impl<Lang> From<ForallUnbounded<Lang>> for ForallBounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
    Top<Lang>: Into<Lang::Type>,
{
    fn from(fu: ForallUnbounded<Lang>) -> ForallBounded<Lang> {
        ForallBounded::new_unbounded(&fu.var, fu.body_ty)
    }
}

impl<Lang> From<ForallUnbounded<Lang>> for Forall<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn from(fu: ForallUnbounded<Lang>) -> Forall<Lang> {
        Forall::new(&fu.var, Kind::Star, fu.body_ty)
    }
}
