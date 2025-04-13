use super::{to_check_err, TypingEnv};
use crate::{
    syntax::{Record, RecordProj},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};
use std::collections::HashMap;

impl<'a> Typecheck<'a> for Record {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let mut tys = HashMap::new();
        for (label, term) in self.records.iter() {
            let ty = term.check(&mut env.clone())?;
            tys.insert(label.clone(), ty);
        }
        Ok(Type::Record(tys))
    }
}

impl<'a> Typecheck<'a> for RecordProj {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let rec_ty = self.record.check(env)?;
        if let Type::Record(tys) = rec_ty {
            tys.get(&self.label)
                .ok_or(to_check_err(ErrorKind::UndefinedLabel(self.label.clone())))
                .cloned()
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: rec_ty.to_string(),
                expected: "Record Type".to_owned(),
            }))
        }
    }
}

#[cfg(test)]
mod record_tests {
    use super::{Record, RecordProj};
    use crate::{
        syntax::{Unit, Zero},
        types::Type,
    };
    use common::Typecheck;
    use std::collections::HashMap;

    #[test]
    fn check_record() {
        let result = Record {
            records: HashMap::from([
                ("label1".to_owned(), Unit.into()),
                ("label2".to_owned(), Zero.into()),
            ]),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Record(HashMap::from([
            ("label1".to_owned(), Type::Unit),
            ("label2".to_owned(), Type::Nat),
        ]));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_proj() {
        let result = RecordProj {
            record: Box::new(
                Record {
                    records: HashMap::from([
                        ("label1".to_owned(), Unit.into()),
                        ("label2".to_owned(), Zero.into()),
                    ]),
                }
                .into(),
            ),
            label: "label1".to_owned(),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }
}
