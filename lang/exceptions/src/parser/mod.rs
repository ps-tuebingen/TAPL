use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/exceptions.pest"]
struct ExceptionsParser;
