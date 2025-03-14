use super::{
    syntax::{
        App, False, If, Lambda, Pair as TermPair, Pred, Proj1, Proj2, RecordProj, Term, True, Tup,
        Zero,
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
        Rule::variable => Ok(Term::Var(p.as_str().to_owned())),
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
        Rule::tup_term => {
            let inner = p.into_inner();
            let tup = pairs_to_tup(inner)?;
            Ok(tup)
        }
        Rule::proj => {
            let inner = p.into_inner();
            pairs_to_proj(inner)
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
    }
    .into())
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

fn pairs_to_proj(mut ps: Pairs<'_, Rule>) -> Result<Term, Error> {
    let term_pair = ps
        .next()
        .ok_or(Error::MissingInput("Projection Term".to_owned()))?;
    let proj_term = pair_to_term(term_pair)?;
    let proj_rule = ps
        .next()
        .ok_or(Error::MissingInput("Projection".to_owned()))?;
    match proj_rule.as_rule() {
        Rule::fst => Ok(Proj1 {
            pair: Box::new(proj_term),
        }
        .into()),
        Rule::snd => Ok(Proj2 {
            pair: Box::new(proj_term),
        }
        .into()),
        Rule::rec_proj => Ok(RecordProj {
            record: Box::new(proj_term),
            label: proj_rule.as_str().to_owned(),
        }
        .into()),
        r => Err(Error::UnexpectedRule {
            found: r,
            expected: Rule::rec_proj,
        }),
    }
}

fn pairs_to_tup(mut ps: Pairs<'_, Rule>) -> Result<Term, Error> {
    let fst_pair = ps
        .next()
        .ok_or(Error::MissingInput("Pair First".to_owned()))?;
    let fst_rule = next_rule(fst_pair, Rule::term)?;
    let fst_term = pair_to_term(fst_rule)?;
    let snd_pair = ps
        .next()
        .ok_or(Error::MissingInput("Pair Second".to_owned()))?;
    let snd_rule = next_rule(snd_pair, Rule::term)?;
    let snd_term = pair_to_term(snd_rule)?;
    let mut rest = vec![];
    while let Some(p) = ps.next() {
        let p_rule = next_rule(p, Rule::term)?;
        let p_term = pair_to_term(p_rule)?;
        rest.push(p_term);
    }
    if rest.is_empty() {
        Ok(TermPair {
            fst: Box::new(fst_term),
            snd: Box::new(snd_term),
        }
        .into())
    } else {
        rest.insert(0, fst_term);
        rest.insert(1, snd_term);
        Ok(Tup { terms: rest }.into())
    }
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
        r => Err(Error::BadRule(r)),
    }
}

fn str_to_ty(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "bool" => Ok(Type::Bool),
        "nat" => Ok(Type::Nat),
        _ => Err(Error::BadType(s.to_owned())),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "true" => Ok(True.into()),
        "false" => Ok(False.into()),
        "zero" => Ok(Zero.into()),
        _ => Err(Error::BadTerm(s.to_owned())),
    }
}
