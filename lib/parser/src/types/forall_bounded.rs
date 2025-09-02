use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, types::ForallBounded};

impl<Lang> Parse for ForallBounded<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::forall_bounded_type;

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<ForallBounded<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Forall Variable", "Forall Super Type", "Forall Body"],
        )?;
        let var_rule = inner.remove(0);
        let mut var_inner = pair_to_n_inner(var_rule, vec!["Forall Variable"])?;
        let var = var_inner.remove(0).as_str().trim();
        let super_rule = inner.remove(0);
        let super_ty = Lang::Type::from_pair(super_rule, ())?;

        let body_rule = inner.remove(0);
        let body_ty = Lang::Type::from_pair(body_rule, ())?;

        Ok(ForallBounded::new(var, super_ty, body_ty))
    }
}
