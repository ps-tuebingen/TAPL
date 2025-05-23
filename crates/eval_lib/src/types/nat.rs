impl<Ty> Normalize<Ty> for Nat<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, _: &mut Self::Env) -> Ty {
        self.into()
    }
}
