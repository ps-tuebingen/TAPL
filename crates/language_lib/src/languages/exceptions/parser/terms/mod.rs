use super::{
    pair_to_n_inner, Error, MissingInput, RemainingInput, Rule, Term, UnexpectedRule,
    UnknownKeyword,
};
use parse::Parse;
use pest::iterators::Pair;
use syntax::terms::{
    App, Exception, False, If, IsZero, Lambda, Num, Pred, Raise, Succ, True, Try, TryWithVal, Unit,
    Variable,
};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_pair = inner
        .next()
        .ok_or(MissingInput::new("Non left-recursive Term"))?;
    let prim_rule = pair_to_n_inner(prim_pair, vec!["Prim Term"])?.remove(0);
    let prim_term = pair_to_prim_term(prim_rule)?;

    let term = match inner.next() {
        Some(p) => {
            let left_rec_rule = pair_to_n_inner(p, vec!["Application Term"])?.remove(0);
            pair_to_leftrec(left_rec_rule, prim_term)?
        }
        None => prim_term,
    };

    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }
    Ok(term)
}

pub fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| UnknownKeyword::new(p.as_str()))?;
            Ok(Term::Num(Num::new(num)))
        }
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        Rule::const_term => str_to_term(p.as_str()),
        Rule::lambda_term => Ok(Lambda::from_pair(p)?.into()),
        Rule::succ_term => Ok(Succ::from_pair(p)?.into()),
        Rule::pred_term => Ok(Pred::from_pair(p)?.into()),
        Rule::iszero_term => Ok(IsZero::from_pair(p)?.into()),
        Rule::if_term => Ok(If::from_pair(p)?.into()),
        Rule::try_term => Ok(Try::from_pair(p)?.into()),
        Rule::try_catch => Ok(TryWithVal::from_pair(p)?.into()),
        Rule::raise_term => Ok(Raise::from_pair(p)?.into()),
        Rule::err_term => Ok(Exception::from_pair(p)?.into()),
        Rule::paren_term => {
            let inner = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(inner)
        }
        r => Err(UnexpectedRule::new(r, "Non Left-recursive Term").into()),
    }
}

pub fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::term => {
            let arg = pair_to_term(p)?;
            Ok(App {
                fun: Box::new(t),
                arg: Box::new(arg),
            }
            .into())
        }
        r => Err(UnexpectedRule::new(r, "Term").into()),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "true" => Ok(True::new().into()),
        "false" => Ok(False::new().into()),
        s => Err(UnknownKeyword::new(s).into()),
    }
}
