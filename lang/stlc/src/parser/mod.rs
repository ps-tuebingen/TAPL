use super::syntax::Term;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/stlc.pest"]
struct StlcParser;

pub fn parse(input: String) -> Result<(), Box<dyn std::error::Error>> {
    let res = StlcParser::parse(Rule::term, &input)?;
    println!("{res}");
    Ok(())
}
