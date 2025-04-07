use crate::{errors::Error, syntax::Term};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "typed_arith.pest"]
struct TypedArithParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = TypedArithParser::parse(Rule::term, &input)?;
    let term_rule = parsed
        .next()
        .ok_or(Error::MissingInput("Program".to_owned()))?;
    let term_inner = pair_to_n_inner(term_rule, vec!["Term"])?.remove(0);
    if let Some(n) = parsed.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    pair_to_term(term_inner)
}

fn pair_to_n_inner<'a>(p: Pair<'a, Rule>, names: Vec<&str>) -> Result<Vec<Pair<'a, Rule>>, Error> {
    let mut inner = p.into_inner();
    let mut pairs = vec![];
    for name in names {
        let next = inner.next().ok_or(Error::MissingInput(name.to_owned()))?;
        pairs.push(next);
    }
    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(pairs)
}

fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::kw_true => Ok(Term::True),
        Rule::kw_false => Ok(Term::False),
        Rule::kw_zero => Ok(Term::Zero),
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            let term_inner = pair_to_n_inner(term_rule, vec!["Term"])?.remove(0);
            pair_to_term(term_inner)
        }
        Rule::succ_term => {
            let inner_rule = pair_to_n_inner(p, vec!["Succ Argument"])?.remove(0);
            let arg = pair_to_term(inner_rule)?;
            Ok(Term::Succ(Box::new(arg)))
        }
        Rule::pred_term => {
            let inner_rule = pair_to_n_inner(p, vec!["Pred Argument"])?.remove(0);
            let arg = pair_to_term(inner_rule)?;
            Ok(Term::Pred(Box::new(arg)))
        }
        Rule::iszero_term => {
            let inner_rule = pair_to_n_inner(p, vec!["IsZero Argument"])?.remove(0);
            let arg = pair_to_term(inner_rule)?;
            Ok(Term::IsZero(Box::new(arg)))
        }
        Rule::if_term => {
            let mut inner = pair_to_n_inner(p, vec!["If Condition", "Then Term", "Else Term"])?;
            let ifc = Box::new(pair_to_term(inner.remove(0))?);
            let then_inner = pair_to_n_inner(inner.remove(0), vec!["If Then Term"])?.remove(0);
            let thent = Box::new(pair_to_term(then_inner)?);
            let else_inner = pair_to_n_inner(inner.remove(0), vec!["If Then Term"])?.remove(0);

            let elset = Box::new(pair_to_term(else_inner)?);
            Ok(Term::If { ifc, thent, elset })
        }
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| Error::UnknownKeyword(p.as_str().to_owned()))?;
            Ok(num.into())
        }
        r => Err(Error::UnexpectedRule {
            found: r,
            expected: "Term".to_owned(),
        }),
    }
}
