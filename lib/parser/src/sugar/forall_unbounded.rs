use crate::{GroupParse, Parse, Rule, pair_span, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{
    TypeVar,
    kinds::Kind,
    language::Language,
    span::Span,
    types::{Forall, ForallBounded, Top},
};

/// Helper struct for parsing unbounded universal types
/// This constructs either [`syntax::types::forall::Forall`] or
/// [`syntax::types::forall_bounded::ForallBounded`] depending on `Lang`
pub struct ForallUnbounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    /// Bound Variable
    var: TypeVar,
    /// Inner Type
    body_ty: Lang::Type,
    /// Source Location
    pub span: Span,
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

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Body"])?;
        let var_rule = inner.remove(0);
        let mut var_inner = pair_to_n_inner(var_rule, vec!["Forall Variable"])?;
        let var = var_inner.remove(0).as_str().trim().to_owned();
        let body_rule = inner.remove(0);
        let body_ty = Lang::Type::from_pair(body_rule, ())?;
        Ok(Self { var, body_ty, span })
    }
}

impl<Lang> From<ForallUnbounded<Lang>> for ForallBounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
    Top<Lang>: Into<Lang::Type>,
{
    fn from(fu: ForallUnbounded<Lang>) -> Self {
        Self::new_unbounded(&fu.var, fu.body_ty, fu.span)
    }
}

impl<Lang> From<ForallUnbounded<Lang>> for Forall<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn from(fu: ForallUnbounded<Lang>) -> Self {
        Self::new(&fu.var, Kind::Star, fu.body_ty, fu.span)
    }
}
