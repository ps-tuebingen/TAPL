use super::{errors::Error, pair_to_n_inner, Rule};
use crate::syntax::Term;
use pest::iterators::Pair;

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(Error::MissingInput("Non Left-Recursive Term".to_owned()))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let prim_term = pair_to_prim_term(prim_inner)?;
    let term = if let Some(n) = inner.next() {
        pair_to_leftrec(n, prim_term)?
    } else {
        prim_term
    };
    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(term)
}

fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::object_creation => pair_to_object_creation(p),
        Rule::cast_term => pair_to_cast_term(p),
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| Error::NotANumber(p.as_str().to_owned()))?;
            Ok(Term::Const(num))
        }
        Rule::variable => Ok(Term::Var(p.as_str().trim().to_owned())),
        r => Err(Error::unexpected(r, "Non Left-Recursive Term")),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::field_access => pair_to_field_access(p, t),
        Rule::method_invocation => pair_to_method_invocation(p, t),
        r => Err(Error::unexpected(r, "Field Access or Method Invocation")),
    }
}

fn pair_to_object_creation(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    inner
        .next()
        .ok_or(Error::MissingInput("New Keyword".to_owned()))?;
    let class_name = inner
        .next()
        .ok_or(Error::MissingInput("Class Name".to_owned()))?
        .as_str()
        .trim()
        .to_owned();
    let mut args = vec![];
    if let Some(n) = inner.next() {
        for arg_pair in n.into_inner() {
            let term = pair_to_term(arg_pair)?;
            args.push(term);
        }
    }
    Ok(Term::New(class_name, args))
}

fn pair_to_cast_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Cast Target", "Cast Term"])?;
    let class_name = inner.remove(0).as_str().trim().to_owned();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Term::Cast(class_name, Box::new(term)))
}

fn pair_to_field_access(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    let var = pair_to_n_inner(p, vec!["Field Name"])?
        .remove(0)
        .as_str()
        .trim()
        .to_owned();
    Ok(Term::FieldProjection(Box::new(t), var))
}

fn pair_to_method_invocation(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let method_name = inner
        .next()
        .ok_or(Error::MissingInput("Method Name".to_owned()))?
        .as_str()
        .trim()
        .to_owned();
    let mut args = vec![];
    if let Some(n) = inner.next() {
        for arg_rule in n.into_inner() {
            let term = pair_to_term(arg_rule)?;
            args.push(term);
        }
    }
    Ok(Term::MethodCall(Box::new(t), method_name, args))
}
