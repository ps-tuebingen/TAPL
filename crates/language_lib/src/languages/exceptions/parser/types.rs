use super::{pair_to_n_inner, Error, Rule, Type};
use common::parse::{UnexpectedRule, UnknownKeyword};
use pest::iterators::Pair;
use syntax::types::{Bool, Fun, Nat, Unit};

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::prim_type => str_to_type(p.as_str()),
        Rule::fun_type => pair_to_fun_ty(p),
        Rule::paren_type => {
            let inner_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(inner_rule)
        }
        r => Err(UnexpectedRule::new(r, "Type").into()),
    }
}

fn str_to_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "nat" => Ok(Nat::new().into()),
        "bool" => Ok(Bool::new().into()),
        s => Err(UnknownKeyword::new(s).into()),
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

    Ok(Fun::new(from_ty, to_ty).into())
}
