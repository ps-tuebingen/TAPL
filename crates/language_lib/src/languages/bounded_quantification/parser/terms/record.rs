use super::{pair_to_n_inner, Error, Rule, Term};
use pest::iterators::Pair;
use syntax::terms::RecordProj;

pub fn pair_to_proj(p: Pair<'_, Rule>, t: Term) -> Result<RecordProj<Term>, Error> {
    let label = pair_to_n_inner(p, vec!["Projection Target"])?
        .remove(0)
        .as_str()
        .trim();
    Ok(RecordProj::new(t, label))
}
