use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::LambdaSub};

impl<Lang> Parse for LambdaSub<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::lambda_sub_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<LambdaSub<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Type Variable", "Super Type", "Type Abstraction Body"],
        )?;
        let var = inner.remove(0).as_str().trim();
        let super_rule = inner.remove(0);
        let sup_ty = Lang::Type::from_pair(super_rule, ())?;
        let body_rule = inner.remove(0);
        let body = Lang::Term::from_pair(body_rule, ())?;
        Ok(LambdaSub::new(var, sup_ty, body))
    }
}
