use super::pair_to_term;
use crate::{
    parser::{errors::Error, get_n_inner, Rule},
    syntax::Fix,
};
use pest::iterators::Pair;

pub fn pair_to_fix(p: Pair<'_, Rule>) -> Result<Fix, Error> {
    println!("trying to parse fix");
    let inner_rule = get_n_inner(p, vec!["Fix Term"])?.remove(0);
    println!("fix inner rule {:?}", inner_rule.as_rule());
    let inner = pair_to_term(inner_rule)?;
    println!("got fix inner {inner}");
    Ok(Fix {
        term: Box::new(inner),
    })
}
