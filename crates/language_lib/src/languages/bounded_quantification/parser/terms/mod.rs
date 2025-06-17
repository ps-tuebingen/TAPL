use super::{
    pair_to_n_inner, Error, MissingInput, ParserError, RemainingInput, Rule, Term, UnexpectedRule,
};
use parse::{sugar::LambdaSubUnbounded, Parse};
use pest::iterators::Pair;
use syntax::terms::{
    App, Lambda, LambdaSub, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack, Variable,
};

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
        Rule::paren_term => pair_to_term(pair_to_n_inner(p, vec!["Term"])?.remove(0)),
        Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
        Rule::lambda_sub_term => Ok(LambdaSub::from_pair(p, ())?.into()),
        Rule::ty_lambda_term => Ok(LambdaSubUnbounded::from_pair(p, ())?.to_lambda_sub().into()),
        Rule::pack_term => Ok(Pack::from_pair(p, ())?.into()),
        Rule::unpack_term => Ok(Unpack::from_pair(p, ())?.into()),
        Rule::record_term => Ok(Record::from_pair(p, ())?.into()),
        Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
        Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
        Rule::number => Ok(Num::from_pair(p, ())?.into()),
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        r => Err(UnexpectedRule::new(r, "Non Left-Recursive Term").into()),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::record_proj => Ok(RecordProj::from_pair(p, t)?.into()),
        Rule::tyapp => Ok(TyApp::from_pair(p, t)?.into()),
        Rule::term => Ok(App::from_pair(p, t)?.into()),
        r => Err(UnexpectedRule::new(r, "Type or Term Application").into()),
    }
}
