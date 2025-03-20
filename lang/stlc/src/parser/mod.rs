use super::{
    syntax::{
        App, False, If, Lambda, Left, Let, Pred, Proj, Record, RecordProj, Right, Succ, Term, True,
        Tup, Unit, Variant, Zero,
    },
    types::Type,
};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::collections::HashMap;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/stlc.pest"]
struct StlcParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = StlcParser::parse(Rule::term, &input)?;
    let term_rule = parsed.next().ok_or(Error::EmptyInput)?;
    if let Some(n) = parsed.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    let next = next_rule(term_rule, Rule::term)?;
    let term = pair_to_term(next)?;
    Ok(term)
}

fn next_rule(p: Pair<'_, Rule>, r: Rule) -> Result<Pair<'_, Rule>, Error> {
    let rule = p.as_rule();
    if rule != r {
        return Err(Error::UnexpectedRule {
            found: rule,
            expected: r,
        });
    }
    let mut inner = p.into_inner();
    let next_rule = inner.next().ok_or(Error::EmptyInput)?;
    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(next_rule)
}

fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::variable => Ok(Term::Var(p.as_str().trim().to_owned())),
        Rule::r#const => str_to_term(p.as_str()),
        Rule::lambda_term => {
            let inner = p.into_inner();
            let lam = pairs_to_lambda(inner)?;
            Ok(lam.into())
        }
        Rule::term_app => {
            let inner = p.into_inner();
            let app = pairs_to_app(inner)?;
            Ok(app.into())
        }
        Rule::if_term => {
            let inner = p.into_inner();
            let ift = pairs_to_if(inner)?;
            Ok(ift.into())
        }
        Rule::let_term => {
            let inner = p.into_inner();
            let lett = pairs_to_let(inner)?;
            Ok(lett.into())
        }
        Rule::tup_term => {
            let inner = p.into_inner();
            let tup = pairs_to_tup(inner)?;
            Ok(tup.into())
        }
        Rule::tup_proj => {
            let mut inner = p.into_inner();
            let proj_pair = inner
                .next()
                .ok_or(Error::MissingInput("Projection Term".to_owned()))?;
            let proj_term = pair_to_term(proj_pair)?;
            let num_pair = inner
                .next()
                .ok_or(Error::MissingInput("Projection Index".to_owned()))?;
            let num = num_pair
                .as_str()
                .parse::<usize>()
                .map_err(|_| Error::BadTerm(num_pair.as_str().to_owned()))?;
            if let Some(n) = inner.next() {
                return Err(Error::RemainingInput(n.as_rule()));
            }
            Ok(Proj {
                tup: Box::new(proj_term),
                ind: num,
            }
            .into())
        }
        Rule::rec_term => {
            let mut inner = p.into_inner();
            let mut records = HashMap::new();
            while let Some(n) = inner.next() {
                let var = n.as_str().to_owned();
                let next_pair = inner
                    .next()
                    .ok_or(Error::MissingInput("Record Term".to_owned()))?;
                let n_rule = next_rule(next_pair, Rule::term)?;
                let n_term = pair_to_term(n_rule)?;
                records.insert(var, n_term);
            }
            Ok(Record { records }.into())
        }
        Rule::rec_proj => {
            let mut inner = p.into_inner();
            let rec_pair = inner
                .next()
                .ok_or(Error::MissingInput("Projection Record".to_owned()))?;
            let rec_term = pair_to_term(rec_pair)?;
            let var_pair = inner
                .next()
                .ok_or(Error::MissingInput("Record Label".to_owned()))?;
            let var = var_pair.as_str().to_owned();
            if let Some(n) = inner.next() {
                return Err(Error::RemainingInput(n.as_rule()));
            }
            Ok(RecordProj {
                record: Box::new(rec_term),
                label: var,
            }
            .into())
        }
        Rule::left_term => {
            let mut inner = p.into_inner();
            let arg_pair = inner
                .next()
                .ok_or(Error::MissingInput("inl argument".to_owned()))?;
            let arg_term = pair_to_term(arg_pair)?;
            let ty_pair = inner
                .next()
                .ok_or(Error::MissingInput("Right Type".to_owned()))?;
            let ty_rule = next_rule(ty_pair, Rule::r#type)?;
            let ty = pair_to_type(ty_rule)?;
            if let Some(n) = inner.next() {
                return Err(Error::RemainingInput(n.as_rule()));
            }
            Ok(Left {
                left_term: Box::new(arg_term),
                right_ty: ty,
            }
            .into())
        }
        Rule::right_term => {
            let mut inner = p.into_inner();
            let arg_pair = inner
                .next()
                .ok_or(Error::MissingInput("inr argument".to_owned()))?;
            let arg_term = pair_to_term(arg_pair)?;
            let ty_pair = inner
                .next()
                .ok_or(Error::MissingInput("Left Type".to_owned()))?;
            let ty_rule = next_rule(ty_pair, Rule::r#type)?;
            let ty = pair_to_type(ty_rule)?;
            Ok(Right {
                right_term: Box::new(arg_term),
                left_ty: ty,
            }
            .into())
        }
        Rule::pred_term => {
            let mut inner = p.into_inner();
            let inner_pair = inner
                .next()
                .ok_or(Error::MissingInput("Pred Argument".to_owned()))?;
            let inner_term = pair_to_term(inner_pair)?;
            if let Some(n) = inner.next() {
                return Err(Error::RemainingInput(n.as_rule()));
            }
            Ok(Pred {
                term: Box::new(inner_term),
            }
            .into())
        }
        Rule::variant_term => {
            let mut inner = p.into_inner();
            let var_pair = inner
                .next()
                .ok_or(Error::MissingInput("Variant Label".to_owned()))?;
            let var = var_pair.as_str().to_owned();
            let term_pair = inner
                .next()
                .ok_or(Error::MissingInput("Variant term".to_owned()))?;
            let term_rule = next_rule(term_pair, Rule::term)?;
            let variant_term = pair_to_term(term_rule)?;
            let ty_pair = inner
                .next()
                .ok_or(Error::MissingInput("Variant Type".to_owned()))?;
            let ty_rule = next_rule(ty_pair, Rule::r#type)?;
            let variant_ty = pair_to_type(ty_rule)?;
            Ok(Variant {
                label: var,
                term: Box::new(variant_term),
                ty: variant_ty,
            }
            .into())
        }
        Rule::succ_term => {
            let mut inner = p.into_inner();
            let inner_pair = inner
                .next()
                .ok_or(Error::MissingInput("Pred Argument".to_owned()))?;
            let inner_term = pair_to_term(inner_pair)?;
            if let Some(n) = inner.next() {
                return Err(Error::RemainingInput(n.as_rule()));
            }
            Ok(Succ {
                term: Box::new(inner_term),
            }
            .into())
        }
        Rule::number => {
            let num_str = p.as_str();
            let num = num_str
                .parse::<i64>()
                .map_err(|_| Error::BadTerm(num_str.to_owned()))?;
            Ok(num.into())
        }
        Rule::paren_term => {
            let mut inner = p.into_inner();
            let next = inner.next().ok_or(Error::EmptyInput)?;
            if let Some(n) = inner.next() {
                return Err(Error::RemainingInput(n.as_rule()));
            }
            let next_rule = next_rule(next, Rule::term)?;
            pair_to_term(next_rule)
        }
        r => Err(Error::BadRule(r)),
    }
}

fn pairs_to_lambda(mut ps: Pairs<'_, Rule>) -> Result<Lambda, Error> {
    let var = ps
        .next()
        .ok_or(Error::MissingInput("Lambda Variable".to_owned()))?
        .as_str();
    let ty_pair = ps
        .next()
        .ok_or(Error::MissingInput("Lambda Annot".to_owned()))?;
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;
    let term_pair = ps
        .next()
        .ok_or(Error::MissingInput("Lambda Body".to_owned()))?;
    let next = next_rule(term_pair, Rule::term)?;
    let term = pair_to_term(next)?;
    if let Some(n) = ps.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(Lambda {
        var: var.to_owned(),
        annot: ty,
        body: Box::new(term),
    })
}

fn pairs_to_app(mut ps: Pairs<'_, Rule>) -> Result<App, Error> {
    let fst = ps
        .next()
        .ok_or(Error::MissingInput("Function Term".to_owned()))?;
    let fst_rule = next_rule(fst, Rule::term)?;
    let fst_term = pair_to_term(fst_rule)?;
    let snd = ps
        .next()
        .ok_or(Error::MissingInput("Function Argument".to_owned()))?;
    let snd_rule = next_rule(snd, Rule::term)?;
    let snd_term = pair_to_term(snd_rule)?;
    if let Some(n) = ps.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(App {
        fun: Box::new(fst_term),
        arg: Box::new(snd_term),
    })
}

fn pairs_to_if(mut ps: Pairs<'_, Rule>) -> Result<If, Error> {
    let if_pair = ps.next().ok_or(Error::MissingInput("If Term".to_owned()))?;
    let if_rule = next_rule(if_pair, Rule::term)?;
    let if_term = pair_to_term(if_rule)?;
    let then_pair = ps
        .next()
        .ok_or(Error::MissingInput("Then Term".to_owned()))?;
    let then_rule = next_rule(then_pair, Rule::term)?;
    let then_term = pair_to_term(then_rule)?;
    let else_pair = ps
        .next()
        .ok_or(Error::MissingInput("Else Term".to_owned()))?;
    let else_rule = next_rule(else_pair, Rule::term)?;
    let else_term = pair_to_term(else_rule)?;
    Ok(If {
        ifc: Box::new(if_term),
        thenc: Box::new(then_term),
        elsec: Box::new(else_term),
    })
}

fn pairs_to_tup(ps: Pairs<'_, Rule>) -> Result<Tup, Error> {
    let mut terms = vec![];
    for p in ps {
        let p_rule = next_rule(p, Rule::term)?;
        let p_term = pair_to_term(p_rule)?;
        terms.push(p_term);
    }
    Ok(Tup { terms })
}

fn pairs_to_let(mut ps: Pairs<'_, Rule>) -> Result<Let, Error> {
    let var_pair = ps
        .next()
        .ok_or(Error::MissingInput("Let Variable".to_owned()))?;
    let var = var_pair.as_str().to_owned();
    let bound_pair = ps
        .next()
        .ok_or(Error::MissingInput("Let Bound Term".to_owned()))?;
    let bound_rule = next_rule(bound_pair, Rule::term)?;
    let bound_term = pair_to_term(bound_rule)?;
    let in_pair = ps
        .next()
        .ok_or(Error::MissingInput("Let In Term".to_owned()))?;
    let in_rule = next_rule(in_pair, Rule::term)?;
    let in_term = pair_to_term(in_rule)?;
    Ok(Let {
        var,
        bound_term: Box::new(bound_term),
        in_term: Box::new(in_term),
    })
}

fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::prim_type => str_to_ty(p.as_str()),
        Rule::fun_type => {
            let mut inner = p.into_inner();
            let from_pair = inner
                .next()
                .ok_or(Error::MissingInput("From Type".to_owned()))?;
            let from_rule = next_rule(from_pair, Rule::r#type)?;
            let from_ty = pair_to_type(from_rule)?;
            let to_pair = inner
                .next()
                .ok_or(Error::MissingInput("To Type".to_owned()))?;
            let to_rule = next_rule(to_pair, Rule::r#type)?;
            let to_ty = pair_to_type(to_rule)?;
            Ok(Type::Fun(Box::new(from_ty), Box::new(to_ty)))
        }
        Rule::prod_type => {
            let mut inner = p.into_inner();
            let fst_pair = inner
                .next()
                .ok_or(Error::MissingInput("Product Type first".to_owned()))?;
            let fst_rule = next_rule(fst_pair, Rule::r#type)?;
            let fst_ty = pair_to_type(fst_rule)?;
            let snd_pair = inner
                .next()
                .ok_or(Error::MissingInput("Product Type Second".to_owned()))?;
            let snd_rule = next_rule(snd_pair, Rule::r#type)?;
            let snd_ty = pair_to_type(snd_rule)?;
            Ok(Type::Prod(Box::new(fst_ty), Box::new(snd_ty)))
        }
        Rule::record_type => {
            let mut recs = HashMap::new();
            let mut inner = p.into_inner();
            while let Some(n) = inner.next() {
                let next_var = n.as_str().to_owned();
                let next_pair = inner
                    .next()
                    .ok_or(Error::MissingInput("Record Type".to_owned()))?;
                let next_rule = next_rule(next_pair, Rule::r#type)?;
                let next_ty = pair_to_type(next_rule)?;
                recs.insert(next_var, next_ty);
            }
            Ok(Type::Record(recs))
        }
        Rule::sum_type => {
            let mut inner = p.into_inner();
            let fst_pair = inner
                .next()
                .ok_or(Error::MissingInput("First Sum Type".to_owned()))?;
            let fst_rule = next_rule(fst_pair, Rule::r#type)?;
            let fst_ty = pair_to_type(fst_rule)?;
            let snd_pair = inner
                .next()
                .ok_or(Error::MissingInput("Second Sum Type".to_owned()))?;
            let snd_rule = next_rule(snd_pair, Rule::r#type)?;
            let snd_ty = pair_to_type(snd_rule)?;
            if let Some(n) = inner.next() {
                return Err(Error::RemainingInput(n.as_rule()));
            }
            Ok(Type::Sum(Box::new(fst_ty), Box::new(snd_ty)))
        }
        Rule::variant_type => {
            let mut inner = p.into_inner();
            let mut variants = HashMap::new();
            while let Some(n) = inner.next() {
                let label = n.as_str().to_owned();
                let next_pair = inner
                    .next()
                    .ok_or(Error::MissingInput("Variant Type".to_owned()))?;
                let n_rule = next_rule(next_pair, Rule::r#type)?;
                let n_ty = pair_to_type(n_rule)?;
                variants.insert(label, n_ty);
            }
            Ok(Type::Variant(variants))
        }
        r => Err(Error::BadRule(r)),
    }
}

fn str_to_ty(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "bool" => Ok(Type::Bool),
        "nat" => Ok(Type::Nat),
        "unit" => Ok(Type::Unit),
        _ => Err(Error::BadType(s.to_owned())),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "true" => Ok(True.into()),
        "false" => Ok(False.into()),
        "zero" => Ok(Zero.into()),
        "unit" => Ok(Unit.into()),
        _ => Err(Error::BadTerm(s.to_owned())),
    }
}
