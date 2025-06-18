use super::{errors::Error, terms::Term};
use parse::{errors::UnexpectedRule, terms::StringTerm, GroupParse, Parse, Rule};
use pest::iterators::Pair;

use syntax::terms::{If, IsZero, Num, Pred, Succ};

impl GroupParse for Term {
    type ParseError = Error;

    const RULE: Rule = Rule::term;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, Self::ParseError> {
        match p.as_rule() {
            Rule::const_term => Ok(StringTerm::new()
                .with_true()
                .with_false()
                .with_zero()
                .from_pair(p)?),
            Rule::if_term => Ok(If::from_pair(p, ())?.into()),
            Rule::number => Ok(Num::from_pair(p, ())?.into()),
            Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
            Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
            Rule::iszero_term => Ok(IsZero::from_pair(p, ())?.into()),
            _ => Err(UnexpectedRule::new(p.as_rule(), "Non Left-Recursive Term").into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, _: Term) -> Result<Self, Self::ParseError> {
        Err(UnexpectedRule::new(p.as_rule(), "Non Left-Recursive Term").into())
    }
}
