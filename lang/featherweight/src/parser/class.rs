use super::{
    constructor::pair_to_ctor, errors::Error, methods::pair_to_method_decl, pair_to_n_inner, Rule,
};
use crate::syntax::{
    ClassDeclaration, ClassName, ConstructorDeclaration, FieldName, MethodDeclaration,
};
use pest::iterators::Pair;

struct ClassBody {
    fields: Vec<(ClassName, FieldName)>,
    ctor: ConstructorDeclaration,
    methods: Vec<MethodDeclaration>,
}

pub fn pair_to_classdecl(p: Pair<'_, Rule>) -> Result<ClassDeclaration, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Class Keyword",
            "Class Name",
            "Extends Keyword",
            "Parent Class",
            "Class Body",
        ],
    )?;
    inner.remove(0);
    let class_name = inner.remove(0).as_str().trim().to_owned();
    inner.remove(0);
    let parent_name = inner.remove(0).as_str().trim().to_owned();
    let body_rule = inner.remove(0);
    let body = pair_to_classbody(body_rule)?;

    Ok(ClassDeclaration {
        name: class_name,
        parent: parent_name,
        fields: body.fields,
        constructor: body.ctor,
        methods: body.methods,
    })
}

fn pair_to_classbody(p: Pair<'_, Rule>) -> Result<ClassBody, Error> {
    let mut fields = vec![];
    let mut mctor = None;
    let mut methods = vec![];
    for r in p.into_inner() {
        match r.as_rule() {
            Rule::field => {
                let mut inner = pair_to_n_inner(r, vec!["Class Name", "Field Name"])?;
                let class_name = inner.remove(0).as_str().trim().to_owned();
                let field_name = inner.remove(0).as_str().trim().to_owned();
                fields.push((class_name, field_name));
            }
            Rule::ctor_decl => {
                let ctor = pair_to_ctor(r)?;
                match mctor {
                    None => mctor = Some(ctor),
                    Some(_) => {
                        return Err(Error::MultipleConstructors {
                            class_name: ctor.name,
                        })
                    }
                }
            }
            Rule::method_decl => {
                let method_decl = pair_to_method_decl(r)?;
                methods.push(method_decl);
            }
            r => return Err(Error::unexpected(r, "Field, Constructor or Method")),
        }
    }
    let ctor = match mctor {
        None => return Err(Error::MissingInput("Constructor Declaration".to_owned())),
        Some(c) => c,
    };
    Ok(ClassBody {
        ctor,
        fields,
        methods,
    })
}
