use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    types::{Forall, ForallBounded, Top, Type},
};

pub struct ForallUnbounded<Ty>
where
    Ty: Type,
{
    var: String,
    body_ty: Ty,
}

impl<Ty> ForallUnbounded<Ty>
where
    Ty: Type,
{
    pub fn to_forall_bounded(self) -> ForallBounded<Ty>
    where
        Top<Ty>: Into<Ty>,
    {
        self.into()
    }

    pub fn to_forall_kinded(self) -> Forall<Ty> {
        self.into()
    }
}

impl<Ty> Parse for ForallUnbounded<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::forall_unbounded_type;

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<ForallUnbounded<Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Body"])?;
        let var_rule = inner.remove(0);
        let mut var_inner = pair_to_n_inner(var_rule, vec!["Forall Variable"])?;
        let var = var_inner.remove(0).as_str().trim().to_owned();
        let body_rule = inner.remove(0);
        let body_ty = Ty::from_pair(body_rule, ())?;
        Ok(ForallUnbounded { var, body_ty })
    }
}

impl<Ty> From<ForallUnbounded<Ty>> for ForallBounded<Ty>
where
    Ty: Type,
    Top<Ty>: Into<Ty>,
{
    fn from(fu: ForallUnbounded<Ty>) -> ForallBounded<Ty> {
        ForallBounded::new_unbounded(&fu.var, fu.body_ty)
    }
}

impl<Ty> From<ForallUnbounded<Ty>> for Forall<Ty>
where
    Ty: Type,
{
    fn from(fu: ForallUnbounded<Ty>) -> Forall<Ty> {
        Forall::new(&fu.var, Kind::Star, fu.body_ty)
    }
}
