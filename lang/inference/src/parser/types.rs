use super::{pair_to_n_inner, Error, Rule};
use crate::types::Type;
use pest::iterators::Pair;

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(Error::MissingInput("Non Left-Recursive Type".to_owned()))?;
    let prim_type = pair_to_prim_type(prim_rule)?;
    let ty = match inner.next() {
        None => prim_type,
        Some(n) => {
            let left_rec_inner = pair_to_n_inner(n, vec!["Left Recursive Type"])?.remove(0);
            pair_to_left_rec_type(left_rec_inner, prim_type)?
        }
    };
    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(ty)
}

fn pair_to_prim_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::const_type => str_to_type(p.as_str()),
        Rule::list_type => pair_to_list_type(p),
        Rule::option_type => pair_to_option_type(p),
        Rule::paren_type => {
            let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(ty_rule)
        }
        r => Err(Error::unexpected(r, "Non Left-Recursive Type")),
    }
}

fn pair_to_left_rec_type(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_type => pair_to_fun_type(p, ty),
        Rule::prod_type => pair_to_prod_type(p, ty),
        Rule::sum_type => pair_to_sum_type(p, ty),
        r => Err(Error::unexpected(r, "Left-Recursive Type")),
    }
}

fn str_to_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Type::Unit),
        "nat" => Ok(Type::Nat),
        "bool" => Ok(Type::Bool),
        s => Err(Error::UnknownKw(s.to_owned())),
    }
}

fn pair_to_list_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["List Keyword", "List Type Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["Non Left-recursive Type"])?.remove(0);
    let arg = pair_to_prim_type(arg_inner)?;
    Ok(Type::List(Box::new(arg)))
}

fn pair_to_option_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Optional Keyword", "Optional Type Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["Non Left-Recursive Type"])?.remove(0);
    let arg = pair_to_prim_type(arg_inner)?;
    Ok(Type::Optional(Box::new(arg)))
}

fn pair_to_fun_type(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let inner_rule = pair_to_n_inner(p, vec!["Function To Type"])?.remove(0);
    let inner_type = pair_to_type(inner_rule)?;
    Ok(Type::Fun(Box::new(ty), Box::new(inner_type)))
}

fn pair_to_prod_type(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let inner_rule = pair_to_n_inner(p, vec!["Product Second Type"])?.remove(0);
    let inner_type = pair_to_type(inner_rule)?;
    Ok(Type::Prod(Box::new(ty), Box::new(inner_type)))
}

fn pair_to_sum_type(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let inner_rule = pair_to_n_inner(p, vec![])?.remove(0);
    let inner_type = pair_to_type(inner_rule)?;
    Ok(Type::Sum(Box::new(ty), Box::new(inner_type)))
}
