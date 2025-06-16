use super::{
    pair_to_n_inner, pair_to_type, Error, MissingInput, ParserError, RemainingInput, Rule, Term,
    Type, UnexpectedRule, UnknownKeyword,
};
use parse::Parse;
use pest::iterators::Pair;
use syntax::terms::{App, Lambda, LambdaSub, Num, Pack, Pred, Record, Succ, Unpack, Variable};

mod lambda_sub;
mod record;
use lambda_sub::{pair_to_tyapp, pair_to_tylambda};
use record::pair_to_proj;

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(<MissingInput as Into<ParserError>>::into(
            MissingInput::new("Non Left-Recursive Term"),
        ))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let prim_term = pair_to_prim_term(prim_inner)?;

    let term = match inner.next() {
        None => prim_term,
        Some(leftrec) => {
            let leftrec_inner = pair_to_n_inner(leftrec, vec!["Left Recursive Term"])?.remove(0);
            pair_to_leftrec(leftrec_inner, prim_term)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(
            <RemainingInput as Into<ParserError>>::into(RemainingInput::new(&format!("{n:?}")))
                .into(),
        );
    }
    Ok(term)
}

fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
        Rule::lambda_sub_term => Ok(LambdaSub::from_pair(p, ())?.into()),
        Rule::ty_lambda_term => pair_to_tylambda(p).map(|lam| lam.into()),
        Rule::pack_term => Ok(Pack::from_pair(p, ())?.into()),
        Rule::unpack_term => Ok(Unpack::from_pair(p, ())?.into()),
        Rule::record_term => Ok(Record::from_pair(p, ())?.into()),
        Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
        Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| UnknownKeyword::new(p.as_str()))?;
            Ok(Num::new(num).into())
        }
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        r => Err(
            <UnexpectedRule as Into<ParserError>>::into(UnexpectedRule::new(
                r,
                "Non Left-Recursive Term",
            ))
            .into(),
        ),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::record_proj => pair_to_proj(p, t).map(|proj| proj.into()),
        Rule::tyapp => pair_to_tyapp(p, t).map(|app| app.into()),
        Rule::term => {
            let arg = pair_to_term(p)?;
            Ok(App {
                fun: Box::new(t),
                arg: Box::new(arg),
            }
            .into())
        }
        r => Err(
            <UnexpectedRule as Into<ParserError>>::into(UnexpectedRule::new(
                r,
                "Type or Term Application",
            ))
            .into(),
        ),
    }
}
