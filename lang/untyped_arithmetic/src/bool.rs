#[derive(Debug, PartialEq, Eq)]
pub enum BoolTerm {
    True,
    False,
    If(Box<BoolTerm>, Box<BoolTerm>, Box<BoolTerm>),
}

impl BoolTerm {
    pub fn is_value(&self) -> bool {
        matches!(self, BoolTerm::True | BoolTerm::False)
    }

    pub fn eval_once(self) -> BoolTerm {
        match self {
            BoolTerm::If(t1, t2, t3) => match *t1 {
                BoolTerm::True => *t2,
                BoolTerm::False => *t3,
                _ => BoolTerm::If(Box::new(t1.eval_once()), t2, t3),
            },
            _ => self,
        }
    }

    pub fn eval(self) -> BoolTerm {
        if self.is_value() {
            self
        } else {
            self.eval_once().eval()
        }
    }
}

#[cfg(test)]
mod bool_tests {
    use super::BoolTerm;

    #[test]
    fn value_true() {
        let result = BoolTerm::True.is_value();
        assert!(result)
    }

    #[test]
    fn value_if() {
        let result = BoolTerm::If(
            Box::new(BoolTerm::False),
            Box::new(BoolTerm::True),
            Box::new(BoolTerm::False),
        )
        .is_value();
        assert!(!result)
    }

    #[test]
    fn eval_true() {
        let result = BoolTerm::True.eval();
        let expected = BoolTerm::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_if() {
        let result = BoolTerm::If(
            Box::new(BoolTerm::False),
            Box::new(BoolTerm::True),
            Box::new(BoolTerm::False),
        )
        .eval();
        let expected = BoolTerm::False;
        assert_eq!(result, expected)
    }
}
