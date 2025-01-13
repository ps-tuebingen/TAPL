use crate::{
    errors::Error,
    syntax::{ClassName, ClassTable, FieldName, MethodName, MethodType, Term, Var},
};

pub fn lookup_fields(
    class: &ClassName,
    ct: &ClassTable,
) -> Result<Vec<(ClassName, FieldName)>, Error> {
    if class == "Object" {
        return Ok(vec![]);
    }

    let decl = ct.get(class).ok_or(Error::ClassNotFound(class.clone()))?;

    let mut fields = decl.fields.clone();
    fields.extend(lookup_fields(&decl.parent, ct)?);
    Ok(fields)
}

pub fn lookup_method_type(
    method: &MethodName,
    class: &ClassName,
    ct: &ClassTable,
) -> Result<MethodType, Error> {
    let decl = ct.get(class).ok_or(Error::ClassNotFound(class.clone()))?;
    match decl.methods.iter().find(|mdecl| mdecl.name == *method) {
        None => lookup_method_type(method, &decl.parent, ct),
        Some(mdecl) => Ok(mdecl.get_type()),
    }
}

pub fn lookup_method_body(
    method: &MethodName,
    class: &ClassName,
    ct: &ClassTable,
) -> Result<(Vec<Var>, Term), Error> {
    let decl = ct.get(class).ok_or(Error::ClassNotFound(class.clone()))?;
    match decl.methods.iter().find(|mdecl| mdecl.name == *method) {
        None => lookup_method_body(method, &decl.parent, ct),
        Some(mdecl) => Ok((
            mdecl.args.iter().map(|arg| arg.1.clone()).collect(),
            mdecl.ret.clone(),
        )),
    }
}

pub fn valid_override(
    method: &MethodName,
    class: &ClassName,
    new_ty: &MethodType,
    ct: &ClassTable,
) -> bool {
    let m_type = if let Ok(ty) = lookup_method_type(method, class, ct) {
        ty
    } else {
        return false;
    };
    let args_eq = m_type
        .args
        .iter()
        .zip(new_ty.args.iter())
        .all(|next| next.0 == next.1);
    let arg_lens_eq = m_type.args.len() == new_ty.args.len();
    let ret_eq = m_type.ret == new_ty.ret;
    ret_eq && arg_lens_eq && args_eq
}

#[cfg(test)]
mod lookup_tests {
    use super::{lookup_fields, lookup_method_body, lookup_method_type};
    use crate::{
        syntax::{MethodType, Term},
        test_common::example_table,
    };

    #[test]
    fn lookup_method_setfst() {
        let result =
            lookup_method_body(&"setfst".to_owned(), &"Pair".to_owned(), &example_table()).unwrap();
        let expected = (
            vec!["newfst".to_owned()],
            Term::New(
                "Pair".to_owned(),
                vec![
                    Term::Var("newfst".to_owned()),
                    Term::FieldProjection(Box::new(Term::Var("this".to_owned())), "snd".to_owned()),
                ],
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn lookup_method_fail() {
        let result = lookup_method_body(&"setsnd".to_owned(), &"Pair".to_owned(), &example_table());
        assert!(result.is_err())
    }

    #[test]
    fn lookup_pair_fields() {
        let result = lookup_fields(&"Pair".to_owned(), &example_table()).unwrap();
        let expected = vec![
            ("Object".to_owned(), "fst".to_owned()),
            ("Object".to_owned(), "snd".to_owned()),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn lookup_fields_fail() {
        let result = lookup_fields(&"LPair".to_owned(), &example_table());
        assert!(result.is_err())
    }

    #[test]
    fn lookup_setfst_type() {
        let result =
            lookup_method_type(&"setfst".to_owned(), &"Pair".to_owned(), &example_table()).unwrap();
        let expected = MethodType {
            args: vec!["Object".to_owned()],
            ret: "Pair".to_owned(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn lookup_type_fail() {
        let result = lookup_method_type(&"setsnd".to_owned(), &"Pair".to_owned(), &example_table());
        assert!(result.is_err())
    }
}
