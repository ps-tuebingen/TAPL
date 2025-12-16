use super::{UntypedArithmetic, terms::Term};
use errors::{UnexpectedRule, parse_error::ParserError};
use parser::{GroupParse, Parse, Rule, pair_span, terms::StringTerm};
use pest::iterators::Pair;

use syntax::terms::{If, IsZero, Num, Pred, Succ};

impl GroupParse for Term {
    const RULE: Rule = Rule::term;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        match p.as_rule() {
            Rule::const_term => Ok(StringTerm::<UntypedArithmetic>::new()
                .with_true(span)
                .with_false(span)
                .with_zero(span)
                .from_pair(&p)?),
            Rule::if_term => Ok(If::<UntypedArithmetic>::from_pair(p, ())?.into()),
            Rule::number => Ok(Num::<UntypedArithmetic>::from_pair(p, ())?.into()),
            Rule::succ_term => Ok(Succ::<UntypedArithmetic>::from_pair(p, ())?.into()),
            Rule::pred_term => Ok(Pred::<UntypedArithmetic>::from_pair(p, ())?.into()),
            Rule::iszero_term => Ok(IsZero::<UntypedArithmetic>::from_pair(p, ())?.into()),
            _ => Err(UnexpectedRule::new(
                &format!("{:?}", p.as_rule()),
                "Non Left-Recursive Term",
                span,
            )
            .into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, _: Self) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        Err(UnexpectedRule::new(
            &format!("{:?}", p.as_rule()),
            "Non Left-Recursive Term",
            span,
        )
        .into())
    }
}
