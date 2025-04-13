use crate::{
    parser::Rule,
    syntax::{Proj, RecordProj, Term},
};
use common::errors::Error;
use pest::iterators::Pair;

pub fn pair_to_proj(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    let target_pair_str = p.as_str().trim();
    let term = match target_pair_str.parse::<usize>() {
        Ok(num) => Proj {
            tup: Box::new(t),
            ind: num,
        }
        .into(),
        Err(_) => RecordProj {
            record: Box::new(t),
            label: target_pair_str.to_owned(),
        }
        .into(),
    };
    Ok(term)
}
