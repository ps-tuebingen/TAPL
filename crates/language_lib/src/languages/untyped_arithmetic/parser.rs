use super::terms::Term;
use parse::{
    GroupParse, Parse, Rule,
    errors::{ParserError, UnexpectedRule},
    terms::StringTerm,
};
use pest::iterators::Pair;

use syntax::terms::{If, IsZero, Num, Pred, Succ};

impl GroupParse for Term {
    const RULE: Rule = Rule::term;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError> {
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

    fn from_pair_leftrec(p: Pair<'_, Rule>, _: Term) -> Result<Self, ParserError> {
        Err(UnexpectedRule::new(p.as_rule(), "Non Left-Recursive Term").into())
    }
}
