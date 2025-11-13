use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::{UnexpectedRule, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::{language::Language, terms::ListCase};

impl<Lang> Parse for ListCase<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::listcase_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec![
                "Case Bound Term",
                "First List Pattern (Nil or Cons)",
                "Second List Pattern (Nil,Cons)",
            ],
        )?;
        let bound_rule = inner.remove(0);
        let bound_term = Lang::Term::from_pair(bound_rule, ())?;
        let pair_nil_pt = inner.remove(0);
        let pair_cons_pt = inner.remove(0);
        let (nil_pair, cons_pair) = match (pair_nil_pt.as_rule(), pair_cons_pt.as_rule()) {
            (Rule::nil_pattern, Rule::cons_pattern) => (pair_nil_pt, pair_cons_pt),
            (Rule::cons_pattern, Rule::nil_pattern) => (pair_cons_pt, pair_nil_pt),
            _ => {
                return Err(UnexpectedRule::new(
                    &format!("{:?}", pair_nil_pt.as_rule()),
                    "List Patterns",
                )
                .into());
            }
        };

        let nil_pair = pair_to_n_inner(nil_pair, vec!["Nil Rhs"])?.remove(0);
        let nil_rhs = Lang::Term::from_pair(nil_pair, ())?;

        let mut cons_inner = pair_to_n_inner(
            cons_pair,
            vec!["Cons Head Variable", "Cons Tail Variable", "Cons Rhs Term"],
        )?;
        let head_var = cons_inner.remove(0).as_str().trim();
        let tail_var = cons_inner.remove(0).as_str().trim();
        let cons_rhs = Lang::Term::from_pair(cons_inner.remove(0), ())?;

        Ok(Self::new(bound_term, nil_rhs, head_var, tail_var, cons_rhs))
    }
}
