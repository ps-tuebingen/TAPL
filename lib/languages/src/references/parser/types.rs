use super::{pair_to_n_inner, to_parse_err, Rule, Type};
use common::{
    errors::{Error, ErrorKind},
    types::{Bool, Fun, Nat, Reference, Unit},
};
use pest::iterators::Pair;

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    if p.as_rule() != Rule::r#type {
        return Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{:?}", p.as_rule()),
            expected: "Type".to_owned(),
        }));
    }
    let inner = pair_to_n_inner(p, vec!["Type"])?.remove(0);
    match inner.as_rule() {
        Rule::prim_type => str_to_prim_type(inner.as_str()),
        Rule::fun_type => pair_to_fun_type(inner),
        Rule::ref_type => pair_to_ref_type(inner),
        Rule::paren_type => {
            let inner = pair_to_n_inner(inner, vec!["Type"])?.remove(0);
            pair_to_type(inner)
        }
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Type".to_owned(),
        })),
    }
}

fn str_to_prim_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "nat" => Ok(Nat::new().into()),
        "bool" => Ok(Bool::new().into()),
        s => Err(to_parse_err(ErrorKind::UnknownKeyword(s.to_owned()))),
    }
}

fn pair_to_fun_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["From Type", "To Type"])?;
    let from_rule = inner.remove(0);
    let from_ty = pair_to_type(from_rule)?;
    let to_rule = inner.remove(0);
    let to_ty = pair_to_type(to_rule)?;
    Ok(Fun::new(from_ty, to_ty).into())
}

fn pair_to_ref_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Ref Keyword", "Ref Type"])?;
    let _ = inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Reference::new(ty).into())
}
