use crate::{parser::Rule, terms::Term};
use common::{
    errors::Error,
    terms::{Projection, RecordProj},
};
use pest::iterators::Pair;

pub fn pair_to_proj(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    let target_pair_str = p.as_str().trim();
    let term = match target_pair_str.parse::<usize>() {
        Ok(num) => Projection::new(t, num).into(),
        Err(_) => RecordProj::new(t, target_pair_str).into(),
    };
    Ok(term)
}
