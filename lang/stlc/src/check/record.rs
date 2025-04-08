use super::{errors::Error, TypingEnv};
use crate::{
    syntax::{Record, RecordProj},
    types::Type,
};
use common::Typecheck;
use std::collections::HashMap;

impl<'a> Typecheck<'a> for Record {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
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
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
        let rec_ty = self.record.check(env)?;
        if let Type::Record(tys) = rec_ty {
            tys.get(&self.label)
                .ok_or(Error::UndefinedLabel {
                    label: self.label.clone(),
                })
                .cloned()
        } else {
            Err(Error::UnexpectedType {
                ty: rec_ty.clone(),
                term: self.clone().into(),
            })
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
