use super::{pair_to_term, to_parse_err};
use crate::{
    parser::{get_n_inner, next_rule, Rule},
    terms::Term,
};
use common::{
    errors::{Error, ErrorKind},
    terms::{variantcase::VariantPattern, SomeCase, SumCase, VariantCase},
};
use pest::iterators::Pair;

#[derive(Debug)]
enum PatternBinding {
    Inl { var: String },
    Inr { var: String },
    Variant { label: String, var: String },
    Something { var: String },
    Nothing,
}

impl PatternBinding {
    fn into_inl(self) -> Result<String, Error> {
        if let PatternBinding::Inl { var } = self {
            Ok(var)
        } else {
            Err(to_parse_err(ErrorKind::NameMismatch {
                found: format!("{self:?}"),
                expected: "inl".to_owned(),
            }))
        }
    }

    fn into_inr(self) -> Result<String, Error> {
        if let PatternBinding::Inr { var } = self {
            Ok(var)
        } else {
            Err(to_parse_err(ErrorKind::NameMismatch {
                found: format!("{self:?}"),
                expected: "inr".to_owned(),
            }))
        }
    }

    fn into_variant(self) -> Result<(String, String), Error> {
        if let PatternBinding::Variant { label, var } = self {
            Ok((label, var))
        } else {
            Err(to_parse_err(ErrorKind::NameMismatch {
                found: format!("{self:?}"),
                expected: "variant".to_owned(),
            }))
        }
    }

    fn into_something(self) -> Result<String, Error> {
        if let PatternBinding::Something { var } = self {
            Ok(var)
        } else {
            Err(to_parse_err(ErrorKind::NameMismatch {
                found: format!("{self:?}"),
                expected: "Something".to_owned(),
            }))
        }
    }

    fn into_nothing(self) -> Result<(), Error> {
        if let PatternBinding::Nothing = self {
            Ok(())
        } else {
            Err(to_parse_err(ErrorKind::NameMismatch {
                found: format!("{self:?}"),
                expected: "Nothing".to_owned(),
            }))
        }
    }
}

struct Pattern {
    bnd: PatternBinding,
    rhs: Term,
}

pub fn pair_to_case(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();

    let bound_pair = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Case Bound Term".to_owned(),
    )))?;
    let bound_term = pair_to_term(bound_pair)?;

    let mut patterns = vec![];
    for pt_pair in inner {
        patterns.push(pair_to_pt(pt_pair)?);
    }
    patterns_to_term(patterns, bound_term)
}

fn pair_to_pt(p: Pair<'_, Rule>) -> Result<Pattern, Error> {
    let mut inner = get_n_inner(p, vec!["Pattern Binding", "Pattern Right-Hand Term"])?;
    let bnd_pair = inner.remove(0);
    let bnd_rule = next_rule(bnd_pair, Rule::pt_binding)?;
    let bnd = pair_to_binding(bnd_rule)?;

    let rhs_pair = inner.remove(0);
    let rhs_term = pair_to_term(rhs_pair)?;
    Ok(Pattern { bnd, rhs: rhs_term })
}

fn pair_to_binding(p: Pair<'_, Rule>) -> Result<PatternBinding, Error> {
    match p.as_rule() {
        Rule::inl_bnd => {
            let var = get_n_inner(p, vec!["Inl Argument in Pattern"])?
                .remove(0)
                .as_str()
                .trim()
                .to_owned();
            Ok(PatternBinding::Inl { var })
        }
        Rule::inr_bnd => {
            let var = get_n_inner(p, vec!["Inr Argument in Patterm"])?
                .remove(0)
                .as_str()
                .trim()
                .to_owned();
            Ok(PatternBinding::Inr { var })
        }
        Rule::variant_bnd => {
            let mut inner = get_n_inner(
                p,
                vec!["Variant Label in Pattern", "Variant Argument in Patterm"],
            )?;
            let label = inner.remove(0).as_str().trim().to_owned();
            let var = inner.remove(0).as_str().trim().to_owned();
            Ok(PatternBinding::Variant { label, var })
        }
        Rule::some_bnd => {
            let var = get_n_inner(p, vec!["Something Argument in Patterm"])?
                .remove(0)
                .as_str()
                .trim()
                .to_owned();
            Ok(PatternBinding::Something { var })
        }
        Rule::none_bnd => {
            let _ = get_n_inner(p, vec![]);
            Ok(PatternBinding::Nothing)
        }
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Pattern Binding".to_owned(),
        })),
    }
}

fn patterns_to_term(mut pts: Vec<Pattern>, bound: Term) -> Result<Term, Error> {
    let pt_fst = pts.remove(0);
    let term = match pt_fst.bnd {
        PatternBinding::Inl { var: left_var } => {
            let inr_pt = pts.remove(0);
            let right_var = inr_pt.bnd.into_inr()?;
            SumCase {
                bound_term: Box::new(bound),
                left_var,
                left_term: Box::new(pt_fst.rhs),
                right_var,
                right_term: Box::new(inr_pt.rhs),
            }
            .into()
        }
        PatternBinding::Inr { var: right_var } => {
            let inl_pt = pts.remove(0);
            let left_var = inl_pt.bnd.into_inl()?;
            SumCase {
                bound_term: Box::new(bound),
                left_var,
                left_term: Box::new(inl_pt.rhs),
                right_var,
                right_term: Box::new(pt_fst.rhs),
            }
            .into()
        }
        PatternBinding::Something { var } => {
            let nothing_pt = pts.remove(0);
            nothing_pt.bnd.into_nothing()?;
            SomeCase::new(bound, nothing_pt.rhs, &var, pt_fst.rhs).into()
        }
        PatternBinding::Nothing => {
            let some_pt = pts.remove(0);
            let some_var = some_pt.bnd.into_something()?;
            SomeCase::new(bound, pt_fst.rhs, &some_var, some_pt.rhs).into()
        }
        PatternBinding::Variant { label, var } => {
            let mut cases = vec![VariantPattern::new(&label, &var, pt_fst.rhs)];
            for pt in pts {
                let (label, bound_var) = pt.bnd.into_variant()?;
                cases.push(VariantPattern::new(&label, &bound_var, pt.rhs))
            }
            return Ok(VariantCase::new(bound, cases).into());
        }
    };

    if !pts.is_empty() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            Rule::pattern
        ))));
    }
    Ok(term)
}
