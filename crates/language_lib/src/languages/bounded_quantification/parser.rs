use super::{errors::Error, terms::Term, types::Type};
use parse::{
    GroupParse, Parse, Rule,
    errors::UnexpectedRule,
    pair_to_n_inner,
    sugar::{ExistsUnbounded, ForallUnbounded, LambdaSubStar},
    types::StringTy,
};
use pest::iterators::Pair;
use syntax::{
    terms::{
        App, Lambda, LambdaSub, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack, Variable,
    },
    types::{ExistsBounded, ForallBounded, Fun, Record as RecordTy, Top, TypeVariable},
};

impl GroupParse for Term {
    type ParseError = Error;

    const RULE: Rule = Rule::term;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Term, Error> {
        match p.as_rule() {
            Rule::paren_term => Self::from_pair(pair_to_n_inner(p, vec!["Term"])?.remove(0), ()),
            Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
            Rule::lambda_sub_term => Ok(LambdaSub::from_pair(p, ())?.into()),
            Rule::ty_lambda_star_term => {
                Ok(LambdaSubStar::from_pair(p, ())?.to_lambda_sub().into())
            }
            Rule::pack_term => Ok(Pack::from_pair(p, ())?.into()),
            Rule::unpack_term => Ok(Unpack::from_pair(p, ())?.into()),
            Rule::record_term => Ok(Record::from_pair(p, ())?.into()),
            Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
            Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
            Rule::number => Ok(Num::from_pair(p, ())?.into()),
            Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
            r => Err(UnexpectedRule::new(r, "Non Left-Recursive Term").into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
        match p.as_rule() {
            Rule::record_proj => Ok(RecordProj::from_pair(p, t)?.into()),
            Rule::tyapp => Ok(TyApp::from_pair(p, t)?.into()),
            Rule::term => Ok(App::from_pair(p, t)?.into()),
            r => Err(UnexpectedRule::new(r, "Type or Term Application").into()),
        }
    }
}

impl GroupParse for Type {
    type ParseError = Error;
    const RULE: Rule = Rule::r#type;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, Self::ParseError> {
        match p.as_rule() {
            Rule::const_type => Ok(StringTy::new().with_nat().from_pair(p)?),
            Rule::top_type_star | Rule::top_type => Ok(Top::new_star().into()),
            Rule::forall_bounded_type => Ok(ForallBounded::from_pair(p, ())?.into()),
            Rule::forall_unbounded_type => Ok(ForallUnbounded::from_pair(p, ())?
                .to_forall_bounded()
                .into()),
            Rule::exists_unbounded_type => Ok(ExistsUnbounded::from_pair(p, ())?
                .to_exists_bounded()
                .into()),
            Rule::exists_bounded_type => Ok(ExistsBounded::from_pair(p, ())?.into()),
            Rule::record_type => Ok(RecordTy::from_pair(p, ())?.into()),
            Rule::paren_type => Self::from_pair(pair_to_n_inner(p, vec!["Type"])?.remove(0), ()),
            Rule::type_variable => Ok(TypeVariable::new(p.as_str().trim()).into()),
            r => Err(UnexpectedRule::new(r, "Non Left-Recursive Type").into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
        match p.as_rule() {
            Rule::fun_type => Ok(Fun::from_pair(p, ty)?.into()),
            r => Err(UnexpectedRule::new(r, "Function Type").into()),
        }
    }
}
