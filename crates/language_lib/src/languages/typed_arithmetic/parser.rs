use super::{errors::Error, terms::Term};
use parse::{
    errors::{MissingInput, RemainingInput, UnexpectedRule, UnknownKeyword},
    LangParser, Parse, Rule,
};
use pest::{iterators::Pair, Parser};
use syntax::terms::{False, If, IsZero, Num, Pred, Succ, True};

impl Parse for Term {
    type ParseError = Error;

    fn parse(input: String) -> Result<Self, Error> {
        parse(input)
    }
}

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = LangParser::parse(Rule::term, &input)?;
    let term_rule = parsed.next().ok_or(MissingInput::new("Program"))?;
    let term_inner = pair_to_n_inner(term_rule, vec!["Term"])?.remove(0);
    if let Some(n) = parsed.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }
    pair_to_term(term_inner)
}

fn pair_to_n_inner<'a>(p: Pair<'a, Rule>, names: Vec<&str>) -> Result<Vec<Pair<'a, Rule>>, Error> {
    let mut inner = p.into_inner();
    let mut pairs = vec![];
    for name in names {
        let next = inner.next().ok_or(MissingInput::new(name))?;
        pairs.push(next);
    }
    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }
    Ok(pairs)
}

fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::kw_true => Ok(True::new().into()),
        Rule::kw_false => Ok(False::new().into()),
        Rule::kw_zero => Ok(Num::new(0).into()),
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            let term_inner = pair_to_n_inner(term_rule, vec!["Term"])?.remove(0);
            pair_to_term(term_inner)
        }
        Rule::succ_term => {
            let inner_rule = pair_to_n_inner(p, vec!["Succ Argument"])?.remove(0);
            let arg = pair_to_term(inner_rule)?;
            Ok(Succ::new(arg).into())
        }
        Rule::pred_term => {
            let inner_rule = pair_to_n_inner(p, vec!["Pred Argument"])?.remove(0);
            let arg = pair_to_term(inner_rule)?;
            Ok(Pred::new(arg).into())
        }
        Rule::iszero_term => {
            let inner_rule = pair_to_n_inner(p, vec!["IsZero Argument"])?.remove(0);
            let arg = pair_to_term(inner_rule)?;
            Ok(IsZero::new(arg).into())
        }
        Rule::if_term => {
            let mut inner = pair_to_n_inner(p, vec!["If Condition", "Then Term", "Else Term"])?;
            let ifc = pair_to_term(inner.remove(0))?;
            let then_inner = pair_to_n_inner(inner.remove(0), vec!["If Then Term"])?.remove(0);
            let thent = pair_to_term(then_inner)?;
            let else_inner = pair_to_n_inner(inner.remove(0), vec!["If Then Term"])?.remove(0);

            let elset = pair_to_term(else_inner)?;
            Ok(If::new(ifc, thent, elset).into())
        }
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| UnknownKeyword::new(p.as_str()))?;
            Ok(Num::new(num).into())
        }
        r => Err(UnexpectedRule::new(r, "Term").into()),
    }
}
