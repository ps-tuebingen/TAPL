use super::{pair_to_n_inner, Error, MissingInput, Rule, Term, UnexpectedRule, UnknownKeyword};
use parse::Parse;
use pest::iterators::Pair;
use syntax::terms::{
    App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True, Unit,
    Unpack, Variable,
};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(MissingInput::new("Non Left-Recursive Term"))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let prim_term = pair_to_primterm(prim_inner)?;

    let term = match inner.next() {
        None => prim_term,
        Some(leftrec) => {
            let leftrec_inner = pair_to_n_inner(leftrec, vec!["Left Recursive Term"])?.remove(0);
            pair_to_leftrec(leftrec_inner, prim_term)?
        }
    };
    Ok(term)
}

fn pair_to_primterm(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::const_term => str_to_term(p.as_str()),
        Rule::lambda_term => Ok(Lambda::from_pair(p)?.into()),
        Rule::pack_term => Ok(Pack::from_pair(p)?.into()),
        Rule::unpack_term => Ok(Unpack::from_pair(p)?.into()),
        Rule::succ_term => Ok(Succ::from_pair(p)?.into()),
        Rule::pred_term => Ok(Pred::from_pair(p)?.into()),
        Rule::iszero_term => Ok(IsZero::from_pair(p)?.into()),
        Rule::fix_term => Ok(Fix::from_pair(p)?.into()),
        Rule::record_term => Ok(Record::from_pair(p)?.into()),
        Rule::if_term => Ok(If::from_pair(p)?.into()),
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| UnknownKeyword::new(p.as_str()))?;
            Ok(Num::new(num).into())
        }
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        _ => Err(UnexpectedRule::new(p.as_rule(), "Non Left-Recursive Term").into()),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::record_proj => {
            let label = pair_to_n_inner(p, vec!["Projection Target"])?
                .remove(0)
                .as_str()
                .trim();
            Ok(RecordProj::new(t, label).into())
        }
        Rule::term => {
            let arg_term = pair_to_term(p)?;
            Ok(App {
                fun: Box::new(t),
                arg: Box::new(arg_term),
            }
            .into())
        }
        _ => Err(UnexpectedRule::new(p.as_rule(), "Left Recursive Term").into()),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "zero" => Ok(Num::new(0).into()),
        "true" => Ok(True::new().into()),
        "false" => Ok(False::new().into()),
        s => Err(UnknownKeyword::new(s).into()),
    }
}
