use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::SumCase};

impl<Lang> Parse for SumCase<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::sumcase_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(
            p,
            vec![
                "Case Bound Term",
                "Left/Right Pattern",
                "Left/Right Patttern",
            ],
        )?;
        let bound_rule = inner.remove(0);
        let bound_term = Lang::Term::from_pair(bound_rule, ())?;
        let fst_rule = inner.remove(0);
        let snd_rule = inner.remove(0);
        let (left_var, right_var, left_term, right_term) =
            pairs_to_sum_patterns::<Lang>(fst_rule, snd_rule)?;
        Ok(Self::new(
            bound_term, &left_var, left_term, &right_var, right_term,
        ))
    }
}

fn pairs_to_sum_patterns<Lang>(
    p1: Pair<'_, Rule>,
    p2: Pair<'_, Rule>,
) -> Result<(String, String, Lang::Term, Lang::Term), ParserError>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    let (left_rule, right_rule) = match p1.as_rule() {
        Rule::left_pattern => (p1, p2),
        _ => (p2, p1),
    };
    let mut left_inner = pair_to_n_inner(left_rule, vec!["Left Variable", "Left Term"])?;
    let left_var = left_inner.remove(0).as_str().to_owned();
    let term_rule = left_inner.remove(0);
    let left_term = Lang::Term::from_pair(term_rule, ())?;
    let mut right_inner = pair_to_n_inner(right_rule, vec!["Right Variable", "Right Term"])?;
    let right_var = right_inner.remove(0).as_str().to_owned();
    let term_rule = right_inner.remove(0);
    let right_term = Lang::Term::from_pair(term_rule, ())?;
    Ok((left_var, right_var, left_term, right_term))
}
