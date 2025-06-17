use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::{LambdaSub, TyApp};
