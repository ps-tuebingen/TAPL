use super::{pair_to_n_inner, pair_to_type, Error, Rule};
use crate::syntax::{App, Const, Lambda, LambdaSub, Pack, Pred, Succ, Term, TyApp, Unpack};
use pest::iterators::Pair;

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(Error::MissingInput("Non Left-Recursive Term".to_owned()))?;
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
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(term)
}

fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::lambda_sub => pair_to_lambda_sub(p).map(|lam| lam.into()),
        Rule::pack_term => pair_to_pack(p).map(|pack| pack.into()),
        Rule::unpack_term => pair_to_unpack(p).map(|unp| unp.into()),
        Rule::rec_term => todo!(),
        Rule::succ_term => pair_to_succ(p).map(|s| s.into()),
        Rule::pred_term => pair_to_pred(p).map(|p| p.into()),
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| Error::UnknownKw(p.as_str().to_owned()))?;
            Ok(Const { i: num }.into())
        }
        Rule::variable => Ok(Term::Var(p.as_str().trim().to_owned())),
        r => Err(Error::unexpected(r, "Non Left-Recursive Term")),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::tyapp => pair_to_tyapp(p, t).map(|app| app.into()),
        Rule::paren_tyapp => {
            let inner = pair_to_n_inner(p, vec!["Type Application"])?.remove(0);
            pair_to_leftrec(inner, t)
        }
        Rule::term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            let arg = pair_to_term(term_rule)?;
            Ok(App {
                fun: Box::new(t),
                arg: Box::new(arg),
            }
            .into())
        }
        r => Err(Error::unexpected(r, "Type or Term Application")),
    }
}

fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Lambda, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Body"])?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let annot_rule = inner.remove(0);
    let annot = pair_to_type(annot_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(Lambda {
        var,
        annot,
        body: Box::new(body),
    })
}

fn pair_to_lambda_sub(p: Pair<'_, Rule>) -> Result<LambdaSub, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Type Variable", "Super Type", "Type Abstraction Body"],
    )?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let super_rule = inner.remove(0);
    let sup_ty = pair_to_type(super_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(LambdaSub {
        var,
        sup_ty,
        body: Box::new(body),
    })
}

fn pair_to_pack(p: Pair<'_, Rule>) -> Result<Pack, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Packed Type", "Packed Term", "As Keyword", "Pack Type"],
    )?;
    let packed_rule = inner.remove(0);
    let packed_ty = pair_to_type(packed_rule)?;

    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;

    inner.remove(0);
    let pack_rule = inner.remove(0);
    let pack_ty = pair_to_type(pack_rule)?;

    Ok(Pack {
        inner_ty: packed_ty,
        term: Box::new(term),
        outer_ty: pack_ty,
    })
}

fn pair_to_unpack(p: Pair<'_, Rule>) -> Result<Unpack, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Let Keyword",
            "Unpack Type Name",
            "Unpack Term Name",
            "Pack Term",
            "In Keyword",
            "Unpack Term",
        ],
    )?;
    inner.remove(0);
    let ty_name = inner.remove(0).as_str().trim().to_owned();
    let term_name = inner.remove(0).as_str().trim().to_owned();
    let pack_rule = inner.remove(0);
    let pack_term = pair_to_term(pack_rule)?;

    inner.remove(0);
    let unpack_rule = inner.remove(0);
    let unpack_term = pair_to_term(unpack_rule)?;
    Ok(Unpack {
        ty_var: ty_name,
        bound_var: term_name,
        bound_term: Box::new(pack_term),
        in_term: Box::new(unpack_term),
    })
}

fn pair_to_tyapp(p: Pair<'_, Rule>, t: Term) -> Result<TyApp, Error> {
    let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(TyApp {
        term: Box::new(t),
        ty,
    })
}

fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Succ Keyword", "Succ Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_term = pair_to_prim_term(arg_rule)?;
    Ok(Succ {
        term: Box::new(arg_term),
    })
}

fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pred Keyword", "Pred Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg = pair_to_prim_term(arg_rule)?;
    Ok(Pred {
        term: Box::new(arg),
    })
}
