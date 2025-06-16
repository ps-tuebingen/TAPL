use std::fmt;

#[derive(Debug)]
pub struct UnexpectedRule<R>
where
    R: fmt::Debug,
{
    found: R,
    expected: String,
}
impl<R> UnexpectedRule<R>
where
    R: fmt::Debug,
{
    pub fn new(found: R, expected: &str) -> UnexpectedRule<R> {
        UnexpectedRule {
            found,
            expected: expected.to_owned(),
        }
    }
}

impl<R> fmt::Display for UnexpectedRule<R>
where
    R: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unexpected rule {:?}, expected: {}",
            self.found, self.expected
        )
    }
}

impl<R> std::error::Error for UnexpectedRule<R> where R: fmt::Debug {}
