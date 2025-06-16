use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::types::{ExistsBounded, Type};

impl<Ty> Parse for ExistsBounded<Ty>
where
    Ty: Type + Parse,
{
    type ParseError = <Ty as Parse>::ParseError;

    fn rule() -> Rule {
        Rule::exists_bounded_type
    }

    fn from_pair(p: Pair<'_, Rule>) -> Result<ExistsBounded<Ty>, Self::ParseError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Exists Variable", "Exists Super Type", "Exists Type"],
        )?;
        let var_rule = inner.remove(0);
        let mut var_inner = pair_to_n_inner(var_rule, vec!["Exists Variable"])?;
        let var = var_inner.remove(0).as_str().trim();

        let super_rule = inner.remove(0);
        let sup_ty = Ty::from_pair(super_rule)?;

        let ty_rule = inner.remove(0);
        let ty = Ty::from_pair(ty_rule)?;

        Ok(ExistsBounded::new(var, sup_ty, ty).into())
    }
}
