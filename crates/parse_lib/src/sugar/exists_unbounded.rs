use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::types::{ExistsBounded, Top, Type};

pub struct ExistsUnbounded<Ty>
where
    Ty: Type,
{
    var: String,
    body_ty: Ty,
}

impl<Ty> ExistsUnbounded<Ty>
where
    Ty: Type,
{
    pub fn to_exists_bounded(self) -> ExistsBounded<Ty>
    where
        Ty: Type,
        Top<Ty>: Into<Ty>,
    {
        self.into()
    }
}

impl<Ty> Parse for ExistsUnbounded<Ty>
where
    Ty: Type + Parse<LeftRecArg = ()>,
{
    type ParseError = <Ty as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::exists_unbounded_type;

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<ExistsUnbounded<Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Exists Variable", "Exists Type"])?;
        let var_rule = inner.remove(0);
        let mut var_inner = pair_to_n_inner(var_rule, vec!["Exists Variable"])?;
        let var = var_inner.remove(0).as_str().trim().to_owned();
        let body_rule = inner.remove(0);
        let body_ty = Ty::from_pair(body_rule, ())?;

        Ok(ExistsUnbounded { var, body_ty })
    }
}

impl<Ty> From<ExistsUnbounded<Ty>> for ExistsBounded<Ty>
where
    Ty: Type,
    Top<Ty>: Into<Ty>,
{
    fn from(eu: ExistsUnbounded<Ty>) -> ExistsBounded<Ty> {
        ExistsBounded::new_unbounded(&eu.var, eu.body_ty)
    }
}
