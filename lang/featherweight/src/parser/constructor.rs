use super::{errors::Error, pair_to_n_inner, Rule};
use crate::syntax::{ClassName, ConstructorDeclaration, FieldName};
use pest::iterators::Pair;

pub fn pair_to_ctor(p: Pair<'_, Rule>) -> Result<ConstructorDeclaration, Error> {
    let mut inner = p.into_inner();
    let class_name = inner
        .next()
        .ok_or(Error::MissingInput("Ctor Class Name".to_owned()))?
        .as_str()
        .trim()
        .to_owned();

    let args_or_super = inner
        .next()
        .ok_or(Error::MissingInput("Super Call".to_owned()))?;
    let mut args = vec![];
    let super_rule;
    if args_or_super.as_rule() == Rule::decl_args {
        args = pair_to_ctor_args(args_or_super)?;
        super_rule = inner
            .next()
            .ok_or(Error::MissingInput("Super Call".to_owned()))?;
    } else {
        super_rule = args_or_super;
    }
    let super_args = pair_to_super_args(super_rule)?;

    let mut fields = vec![];
    for field_rule in inner {
        let (this_field, assigned) = pair_to_field_assignment(field_rule)?;
        if args.iter().any(|(_, name)| *name == assigned) {
            return Err(Error::UnknownVariable(assigned));
        }
        fields.push(this_field);
    }

    Ok(ConstructorDeclaration {
        name: class_name,
        super_args,
        self_args: args,
        self_fields: fields,
    })
}

fn pair_to_ctor_args(p: Pair<'_, Rule>) -> Result<Vec<(ClassName, FieldName)>, Error> {
    let mut args = vec![];
    for arg in p.into_inner() {
        let mut inner = pair_to_n_inner(arg, vec!["Class Name", "Arg Name"])?;
        let class_name = inner.remove(0).as_str().trim().to_owned();
        let var_name = inner.remove(0).as_str().trim().to_owned();
        args.push((class_name, var_name));
    }
    Ok(args)
}

fn pair_to_super_args(p: Pair<'_, Rule>) -> Result<Vec<FieldName>, Error> {
    let mut inner = p.into_inner();
    inner
        .next()
        .ok_or(Error::MissingInput("Super Call".to_owned()))?;
    let method_args = if let Some(n) = inner.next() {
        n
    } else {
        return Ok(vec![]);
    };

    let mut args = vec![];
    for method_arg in method_args.into_inner() {
        let method_name = method_arg.as_str().trim().to_owned();
        args.push(method_name);
    }
    Ok(args)
}

fn pair_to_field_assignment(p: Pair<'_, Rule>) -> Result<(FieldName, String), Error> {
    let mut inner = pair_to_n_inner(p, vec!["This Keyword", "Field Name", "Assigned Variable"])?;
    inner.remove(0);
    let field_name = inner.remove(0).as_str().trim().to_owned();
    let var_name = inner.remove(0).as_str().trim().to_owned();
    Ok((field_name, var_name))
}
