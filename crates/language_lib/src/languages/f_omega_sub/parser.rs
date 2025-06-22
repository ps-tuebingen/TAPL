use super::{errors::Error, terms::Term, types::Type};
use parse::{
    GroupParse, Parse, Rule,
    errors::UnexpectedRule,
    pair_to_n_inner,
    sugar::{ExistsUnbounded, ForallUnbounded, LambdaSubStar, OpLambdaUnbounded},
    terms::StringTerm,
    types::StringTy,
};
use pest::iterators::Pair;
use syntax::{
    terms::{
        App, Lambda, LambdaSub, Let, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack,
        Variable,
    },
    types::{
        Exists, ExistsBounded, ForallBounded, Fun, OpApp, OpLambda, OpLambdaSub,
        Record as RecordTy, Top, TypeVariable,
    },
};

//pub mod terms;
//pub mod types;

impl GroupParse for Term {
    type ParseError = Error;

    const RULE: Rule = Rule::term;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Term, Error> {
        match p.as_rule() {
            Rule::paren_term => Self::from_pair(pair_to_n_inner(p, vec!["Term"])?.remove(0), ()),
            Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
            Rule::let_term => Ok(Let::from_pair(p, ())?.into()),
            Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
            Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
            Rule::lambda_sub_term => Ok(LambdaSub::from_pair(p, ())?.into()),
            Rule::ty_lambda_star_term => {
                Ok(LambdaSubStar::from_pair(p, ())?.to_lambda_sub().into())
            }
            Rule::pack_term => Ok(Pack::from_pair(p, ())?.into()),
            Rule::unpack_term => Ok(Unpack::from_pair(p, ())?.into()),
            Rule::record_term => Ok(Record::from_pair(p, ())?.into()),
            Rule::variable => Ok(Variable::from_pair(p, ())?.into()),
            Rule::const_term => Ok(StringTerm::new().with_zero().from_pair(p)?),
            Rule::number => Ok(Num::from_pair(p, ())?.into()),
            _ => Err(UnexpectedRule::new(p.as_rule(), "Non Left-Recursive Term").into()),
        }
    }
    fn from_pair_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
        match p.as_rule() {
            Rule::tyapp => Ok(TyApp::from_pair(p, t)?.into()),
            Rule::record_proj => Ok(RecordProj::from_pair(p, t)?.into()),
            Rule::term => Ok(App::from_pair(p, t)?.into()),
            _ => Err(UnexpectedRule::new(p.as_rule(), "Left Recursive Term").into()),
        }
    }
}

impl GroupParse for Type {
    type ParseError = Error;
    const RULE: Rule = Rule::r#type;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Type, Error> {
        match p.as_rule() {
            Rule::paren_type => Self::from_pair(pair_to_n_inner(p, vec!["Type"])?.remove(0), ()),
            Rule::const_type => Ok(StringTy::new().with_nat().from_pair(p)?),
            Rule::top_type => Ok(Top::from_pair(p, ())?.into()),
            Rule::forall_bounded_type => Ok(ForallBounded::from_pair(p, ())?.into()),
            Rule::forall_unbounded_type => Ok(ForallUnbounded::from_pair(p, ())?
                .to_forall_bounded()
                .into()),
            Rule::op_lambda_type => Ok(OpLambda::from_pair(p, ())?.to_oplambda_unbounded().into()),
            Rule::op_lambda_star_type => Ok(OpLambdaUnbounded::from_pair(p, ())?
                .to_oplambda_sub()
                .into()),
            Rule::op_lambda_sub_type => Ok(OpLambdaSub::from_pair(p, ())?.into()),
            Rule::exists_bounded_type => Ok(ExistsBounded::from_pair(p, ())?.into()),
            Rule::exists_unbounded_type => Ok(ExistsUnbounded::from_pair(p, ())?
                .to_exists_bounded()
                .into()),
            Rule::exists_kinded_type => Ok(Exists::from_pair(p, ())?.to_exists_bounded().into()),
            Rule::record_type => Ok(RecordTy::from_pair(p, ())?.into()),
            Rule::type_variable => Ok(TypeVariable::from_pair(p, ())?.into()),
            _ => Err(UnexpectedRule::new(p.as_rule(), "Non Left-Recursive Type").into()),
        }
    }
    fn from_pair_leftrec(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
        match p.as_rule() {
            Rule::fun_type => Ok(Fun::from_pair(p, ty)?.into()),
            Rule::op_app_type => Ok(OpApp::from_pair(p, ty)?.into()),
            _ => Err(UnexpectedRule::new(p.as_rule(), "Left Recursive Type").into()),
        }
    }
}
