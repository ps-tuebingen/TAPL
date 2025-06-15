use super::{
    get_n_inner, pair_to_type, Error, MissingInput, RemainingInput, Rule, Term, Type,
    UnexpectedRule, UnknownKeyword,
};
use pest::iterators::Pair;
use syntax::terms::{
    variantcase::VariantPattern, App, False, Num, Projection, RecordProj, SomeCase, SumCase, True,
    Unit, Variable, VariantCase,
};

mod ascribe;
mod fix;
mod ift;
mod lambda;
mod lett;
mod list;
mod nat;
mod optional;
mod record;
mod sum;
mod tup;
mod variant;
use ascribe::pair_to_ascribe;
use fix::pair_to_fix;
use ift::pair_to_if;
use lambda::pair_to_lambda;
use lett::pair_to_let;
use list::{pair_to_cons, pair_to_head, pair_to_isnil, pair_to_nil, pair_to_tail};
use nat::{pair_to_isz, pair_to_num, pair_to_pred, pair_to_succ};
use optional::{pair_to_none, pair_to_some};
use record::pair_to_rec;
use sum::{pair_to_left, pair_to_right};
use tup::pair_to_tup;
use variant::pair_to_variant;

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    if p.as_rule() != Rule::term {
        return Err(UnexpectedRule::new(p.as_rule(), "Term").into());
    }

    let mut inner = p.into_inner();
    let prim_term_pair = inner.next().ok_or(MissingInput::new("Term"))?;
    let prim_term_rule = get_n_inner(prim_term_pair, vec!["Prim Term"])?.remove(0);
    let prim_term = pair_to_primterm(prim_term_rule)?;

    let term = if let Some(p) = inner.next() {
        pair_to_leftrec(p, prim_term)?
    } else {
        prim_term
    };

    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }
    Ok(term)
}

pub fn pair_to_primterm(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        Rule::const_term => const_to_term(p.as_str()),
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::some_term => pair_to_some(p).map(|som| som.into()),
        Rule::none_term => pair_to_none(p).map(|non| non.into()),
        Rule::fix_term => pair_to_fix(p).map(|fix| fix.into()),
        Rule::if_term => pair_to_if(p).map(|ift| ift.into()),
        Rule::let_term => pair_to_let(p).map(|lett| lett.into()),
        Rule::tuple_term => pair_to_tup(p).map(|tup| tup.into()),
        Rule::record_term => pair_to_rec(p).map(|rec| rec.into()),
        Rule::left_term => pair_to_left(p).map(|lft| lft.into()),
        Rule::right_term => pair_to_right(p).map(|rgt| rgt.into()),
        Rule::pred_term => pair_to_pred(p).map(|pred| pred.into()),
        Rule::variant_term => pair_to_variant(p).map(|v| v.into()),
        Rule::succ_term => pair_to_succ(p).map(|s| s.into()),
        Rule::iszero_term => pair_to_isz(p).map(|isz| isz.into()),
        Rule::cons_term => pair_to_cons(p).map(|cons| cons.into()),
        Rule::nil_term => pair_to_nil(p).map(|nil| nil.into()),
        Rule::isnil_term => pair_to_isnil(p).map(|isn| isn.into()),
        Rule::head_term => pair_to_head(p).map(|hd| hd.into()),
        Rule::tail_term => pair_to_tail(p).map(|tl| tl.into()),
        Rule::number => pair_to_num(p),
        Rule::paren_term => paren_to_term(p),
        Rule::variantcase_term => pair_to_variantcase(p).map(|case| case.into()),
        Rule::sumcase_term => pair_to_sumcase(p).map(|case| case.into()),
        Rule::somecase_term => pair_to_somecase(p).map(|case| case.into()),
        r => Err(UnexpectedRule::new(r, "Non Left-Recursive Term").into()),
    }
}
pub fn pair_to_variantcase(p: Pair<'_, Rule>) -> Result<VariantCase<Term>, Error> {
    let mut inner = p.into_inner();
    let bound_rule = inner.next().ok_or(MissingInput::new("Case Bound Term"))?;
    let bound_term = pair_to_term(bound_rule)?;
    let mut patterns = vec![];
    for pattern_rule in inner {
        patterns.push(pair_to_variantpattern(pattern_rule)?);
    }
    Ok(VariantCase::new(bound_term, patterns))
}
fn pair_to_variantpattern(p: Pair<'_, Rule>) -> Result<VariantPattern<Term>, Error> {
    let mut inner = get_n_inner(
        p,
        vec![
            "Variant Pattern Label",
            "Variant Pattern Var",
            "Variant Pattern Right-Hand Side",
        ],
    )?;
    let label = inner.remove(0).as_str().trim();
    let var = inner.remove(0).as_str().trim();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(VariantPattern::new(label, var, term))
}

fn pair_to_sumcase(p: Pair<'_, Rule>) -> Result<SumCase<Term>, Error> {
    let mut inner = get_n_inner(
        p,
        vec![
            "Case Bound Term",
            "Left/Right Pattern",
            "Left/Right Patttern",
        ],
    )?;
    let bound_rule = inner.remove(0);
    let bound_term = pair_to_term(bound_rule)?;
    let fst_rule = inner.remove(0);
    let snd_rule = inner.remove(0);
    let (left_var, right_var, left_term, right_term) = pairs_to_sum_patterns(fst_rule, snd_rule)?;
    Ok(SumCase::new(
        bound_term, &left_var, left_term, &right_var, right_term,
    ))
}

fn pairs_to_sum_patterns(
    p1: Pair<'_, Rule>,
    p2: Pair<'_, Rule>,
) -> Result<(String, String, Term, Term), Error> {
    let (left_rule, right_rule) = match p1.as_rule() {
        Rule::left_pattern => (p1, p2),
        _ => (p2, p1),
    };
    let mut left_inner = get_n_inner(left_rule, vec!["Left Variable", "Left Term"])?;
    let left_var = left_inner.remove(0).as_str().to_owned();
    let term_rule = left_inner.remove(0);
    let left_term = pair_to_term(term_rule)?;
    let mut right_inner = get_n_inner(right_rule, vec!["Right Variable", "Right Term"])?;
    let right_var = right_inner.remove(0).as_str().to_owned();
    let term_rule = right_inner.remove(0);
    let right_term = pair_to_term(term_rule)?;
    Ok((left_var, right_var, left_term, right_term))
}

fn pair_to_somecase(p: Pair<'_, Rule>) -> Result<SomeCase<Term>, Error> {
    let mut inner = get_n_inner(
        p,
        vec!["Case Bound term", "Some/None Pattern", "Some/None Pattern"],
    )?;
    let bound_rule = inner.remove(0);
    let bound_term = pair_to_term(bound_rule)?;
    let fst_pt = inner.remove(0);
    let snd_pt = inner.remove(0);
    let (some_rule, none_rule) = match fst_pt.as_rule() {
        Rule::some_pattern => (fst_pt, snd_pt),
        _ => (snd_pt, fst_pt),
    };
    let none_inner = get_n_inner(none_rule, vec!["None Term"])?.remove(0);
    let none_term = pair_to_term(none_inner)?;
    let mut some_inner = get_n_inner(some_rule, vec!["Some Variable", "Some Term"])?;
    let some_var = some_inner.remove(0).as_str().to_owned();
    let some_term_rule = some_inner.remove(0);
    let some_term = pair_to_term(some_term_rule)?;
    Ok(SomeCase::new(bound_term, none_term, &some_var, some_term))
}

fn const_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "true" => Ok(True::new().into()),
        "false" => Ok(False::new().into()),
        "zero" => Ok(Num::new(0).into()),
        "unit" => Ok(Unit::new().into()),
        _ => Err(UnknownKeyword::new(s).into()),
    }
}

fn paren_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let term_rule = get_n_inner(p, vec!["Term"])?.remove(0);
    pair_to_term(term_rule)
}

pub fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::left_rec => {
            let next = get_n_inner(p, vec!["Left Recursive Term"])?.remove(0);
            match next.as_rule() {
                Rule::ascription => pair_to_ascribe(next, t).map(|asc| asc.into()),
                Rule::projection => pair_to_proj(next, t).map(|proj| proj.into()),
                Rule::record_proj => pair_to_recordproj(next, t).map(|proj| proj.into()),
                Rule::term => {
                    let arg = pair_to_term(next)?;
                    Ok(App {
                        fun: Box::new(t),
                        arg: Box::new(arg),
                    }
                    .into())
                }
                r => Err(UnexpectedRule::new(r, "Ascription, Projection or Term").into()),
            }
        }
        Rule::EOI => Ok(t),
        r => Err(UnexpectedRule::new(r, "Left Recursive Term or End of Input").into()),
    }
}

pub fn pair_to_recordproj(p: Pair<'_, Rule>, t: Term) -> Result<RecordProj<Term>, Error> {
    let label = get_n_inner(p, vec!["Projection Target"])?
        .remove(0)
        .as_str()
        .trim();
    Ok(RecordProj::new(t, label))
}

pub fn pair_to_proj(p: Pair<'_, Rule>, t: Term) -> Result<Projection<Term>, Error> {
    let num_str = get_n_inner(p, vec!["Projection Target"])?
        .remove(0)
        .as_str()
        .trim();
    let num = num_str
        .parse::<usize>()
        .map_err(|_| UnknownKeyword::new(num_str))?;
    Ok(Projection::new(t, num))
}
