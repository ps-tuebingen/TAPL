use super::{errors::Error, pair_to_n_inner, terms::pair_to_term, Rule};
use crate::syntax::{ClassName, MethodDeclaration, MethodName, Var};
use pest::iterators::Pair;

pub fn pair_to_method_decl(p: Pair<'_, Rule>) -> Result<MethodDeclaration, Error> {
    let mut inner = p.into_inner();
    let head = inner
        .next()
        .ok_or(Error::MissingInput("Methd Head".to_owned()))?;
    let (class_name, method_name) = pair_to_method_head(head)?;

    let args_or_return = inner
        .next()
        .ok_or(Error::MissingInput("Method Return".to_owned()))?;

    let mut args = vec![];
    if args_or_return.as_rule() == Rule::decl_args {
        args = pair_to_method_args(args_or_return)?;
        inner
            .next()
            .ok_or(Error::MissingInput("Method Return".to_owned()))?;
    }
    let return_rule = inner
        .next()
        .ok_or(Error::MissingInput("Method Return".to_owned()))?;
    let ret = pair_to_term(return_rule)?;
    Ok(MethodDeclaration {
        class: class_name,
        name: method_name,
        args,
        ret,
    })
}

fn pair_to_method_head(p: Pair<'_, Rule>) -> Result<(ClassName, MethodName), Error> {
    let mut inner = pair_to_n_inner(p, vec!["Class Name", "Method Name"])?;
    let class_name = inner.remove(0).as_str().trim().to_owned();
    let method_name = inner.remove(0).as_str().trim().to_owned();
    Ok((class_name, method_name))
}

fn pair_to_method_args(p: Pair<'_, Rule>) -> Result<Vec<(ClassName, Var)>, Error> {
    let mut args = vec![];
    for arg_rule in p.into_inner() {
        let mut inner = pair_to_n_inner(arg_rule, vec!["Class Name", "Variable Name"])?;
        let class_name = inner.remove(0).as_str().trim().to_owned();
        let var_name = inner.remove(0).as_str().trim().to_owned();
        args.push((class_name, var_name));
    }
    Ok(args)
}
