use super::{UntypedLambda, terms::Term};
use errors::{UnexpectedRule, parse_error::ParserError};
use parser::{GroupParse, Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::terms::{App, UntypedLambda as UntypedLambdaT, Variable};

impl GroupParse for Term {
    const RULE: Rule = Rule::term;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError> {
        match p.as_rule() {
            Rule::paren_term => {
                Self::from_pair(pair_to_n_inner(p, vec!["Paren Term Inner"])?.remove(0), ())
            }
            Rule::variable => Ok(Variable::<UntypedLambda>::from_pair(p, ())?.into()),
            Rule::untyped_lambda_term => {
                Ok(UntypedLambdaT::<UntypedLambda>::from_pair(p, ())?.into())
            }
            _ => Err(
                UnexpectedRule::new(&format!("{:?}", p.as_rule()), "Non Left-Recursive Term")
                    .into(),
            ),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Self, ParserError> {
        match p.as_rule() {
            Rule::term => Ok(App::<UntypedLambda>::from_pair(p, t)?.into()),
            _ => Err(UnexpectedRule::new(&format!("{:?}", p.as_rule()), "Aplication").into()),
        }
    }
}
