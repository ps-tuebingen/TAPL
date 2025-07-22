use crate::{Parse, Rule, pair_to_n_inner};
use errors::{UnexpectedRule, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::terms::{ListCase, Term};

impl<T> Parse for ListCase<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::listcase_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<ListCase<T>, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec![
                "Case Bound Term",
                "First List Pattern (Nil or Cons)",
                "Second List Pattern (Nil,Cons)",
            ],
        )?;
        let bound_rule = inner.remove(0);
        let bound_term = T::from_pair(bound_rule, ())?;
        let pt_fst_pair = inner.remove(0);
        let pt_rst_pair = inner.remove(0);
        let (nil_pair, cons_pair) = match (pt_fst_pair.as_rule(), pt_rst_pair.as_rule()) {
            (Rule::nil_pattern, Rule::cons_pattern) => (pt_fst_pair, pt_rst_pair),
            (Rule::cons_pattern, Rule::nil_pattern) => (pt_rst_pair, pt_fst_pair),
            _ => {
                return Err(UnexpectedRule::new(
                    &format!("{:?}", pt_fst_pair.as_rule()),
                    "List Patterns",
                )
                .into());
            }
        };

        let nil_pair = pair_to_n_inner(nil_pair, vec!["Nil Rhs"])?.remove(0);
        let nil_rhs = T::from_pair(nil_pair, ())?;

        let mut cons_inner = pair_to_n_inner(
            cons_pair,
            vec!["Cons Head Variable", "Cons Tail Variable", "Cons Rhs Term"],
        )?;
        let head_var = cons_inner.remove(0).as_str().trim();
        let tail_var = cons_inner.remove(0).as_str().trim();
        let cons_rhs = T::from_pair(cons_inner.remove(0), ())?;

        Ok(ListCase::new(
            bound_term, nil_rhs, head_var, tail_var, cons_rhs,
        ))
    }
}
