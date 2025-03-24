use super::{errors::Error, pair_to_n_inner, Rule};
use crate::types::Type;
use pest::iterators::Pair;

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::prim_type => str_to_type(p.as_str()),
        Rule::fun_type => pair_to_fun_ty(p),
        Rule::paren_type => {
            let inner_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(inner_rule)
        }
        r => Err(Error::unexpected(r, "Type")),
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

fn pair_to_fun_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["From Type", "To Type"])?;
    let from_pair = inner.remove(0);
    let from_rule = pair_to_n_inner(from_pair, vec!["Type"])?.remove(0);
    let from_ty = pair_to_type(from_rule)?;
    let to_pair = inner.remove(0);
    let to_rule = pair_to_n_inner(to_pair, vec!["Type"])?.remove(0);
    let to_ty = pair_to_type(to_rule)?;

    Ok(Type::Fun {
        from: Box::new(from_ty),
        to: Box::new(to_ty),
    })
}
