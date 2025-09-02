use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, types::ExistsBounded};

impl<Lang> Parse for ExistsBounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::exists_bounded_type;

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<ExistsBounded<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Exists Variable", "Exists Super Type", "Exists Type"],
        )?;
        let var_rule = inner.remove(0);
        let mut var_inner = pair_to_n_inner(var_rule, vec!["Exists Variable"])?;
        let var = var_inner.remove(0).as_str().trim();

        let super_rule = inner.remove(0);
        let sup_ty = Lang::Type::from_pair(super_rule, ())?;

        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;

        Ok(ExistsBounded::new(var, sup_ty, ty))
    }
}
