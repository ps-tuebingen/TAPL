use super::{Existential, terms::Term, types::Type};
use errors::{UnexpectedRule, UnknownKeyword, parse_error::ParserError};
use parser::{
    GroupParse, Parse, Rule, pair_span, pair_to_n_inner, sugar::ExistsUnbounded, terms::StringTerm,
    types::StringTy,
};
use pest::iterators::Pair;
use syntax::{
    terms::{
        App, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, Unpack, Variable,
    },
    types::{Exists, Fun, Record as RecordTy, TypeVariable},
};

impl GroupParse for Term {
    const RULE: Rule = Rule::term;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        match p.as_rule() {
            Rule::const_term => Ok(StringTerm::<Existential>::new()
                .with_unit(span)
                .with_zero(span)
                .with_true(span)
                .with_false(span)
                .from_pair(&p)?),
            Rule::paren_term => Self::from_pair(pair_to_n_inner(p, vec!["Term"])?.remove(0), ()),
            Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
            Rule::pack_term => Ok(Pack::from_pair(p, ())?.into()),
            Rule::unpack_term => Ok(Unpack::from_pair(p, ())?.into()),
            Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
            Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
            Rule::iszero_term => Ok(IsZero::from_pair(p, ())?.into()),
            Rule::fix_term => Ok(Fix::from_pair(p, ())?.into()),
            Rule::record_term => Ok(Record::from_pair(p, ())?.into()),
            Rule::if_term => Ok(If::from_pair(p, ())?.into()),
            Rule::number => {
                let num = p
                    .as_str()
                    .trim()
                    .parse::<i64>()
                    .map_err(|_| UnknownKeyword::new(p.as_str(), span))?;
                Ok(Num::new(num, span).into())
            }
            Rule::variable => Ok(Variable::new(p.as_str().trim(), span).into()),
            _ => Err(UnexpectedRule::new(
                &format!("{:?}", p.as_rule(),),
                "Non Left-Recursive Term",
                span,
            )
            .into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, t: Self) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        match p.as_rule() {
            Rule::record_proj => Ok(RecordProj::from_pair(p, t)?.into()),
            Rule::term => Ok(App::from_pair(p, t)?.into()),
            _ => {
                Err(
                    UnexpectedRule::new(&format!("{:?}", p.as_rule()), "Left Recursive Term", span)
                        .into(),
                )
            }
        }
    }
}

impl GroupParse for Type {
    const RULE: Rule = Rule::r#type;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        match p.as_rule() {
            Rule::const_type => Ok(StringTy::<Existential>::new()
                .with_nat(span)
                .with_unit(span)
                .with_bool(span)
                .from_pair(&p)?),
            Rule::paren_type => Self::from_pair(pair_to_n_inner(p, vec!["Type"])?.remove(0), ()),
            Rule::exists_unbounded_type => {
                Ok(ExistsUnbounded::from_pair(p, ())?.to_exists_kinded().into())
            }
            Rule::exists_kinded_type => Ok(Exists::from_pair(p, ())?.into()),
            Rule::record_type => Ok(RecordTy::from_pair(p, ())?.into()),
            Rule::type_variable => Ok(TypeVariable::from_pair(p, ())?.into()),
            _ => Err(UnexpectedRule::new(
                &format!("{:?}", p.as_rule()),
                "Non Left-Recursive Type",
                span,
            )
            .into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, ty: Self) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        match p.as_rule() {
            Rule::fun_type => Ok(Fun::from_pair(p, ty)?.into()),
            _ => {
                Err(
                    UnexpectedRule::new(&format!("{:?}", p.as_rule()), "Left Recursive Term", span)
                        .into(),
                )
            }
        }
    }
}
