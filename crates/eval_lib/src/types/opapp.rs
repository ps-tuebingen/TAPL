impl<Ty> Normalize<Ty> for OpApp<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        let fun_norm = self.fun.normalize(env);
        if let Ok(oplam) = fun_norm.clone().into_oplambda() {
            oplam.body.subst_type(&oplam.var, &self.arg).normalize(env)
        } else if let Ok(oplam) = fun_norm.clone().into_oplambdasub() {
            oplam.body.subst_type(&oplam.var, &self.arg).normalize(env)
        } else {
            OpApp {
                fun: Box::new(fun_norm),
                arg: Box::new(self.arg.normalize(env)),
            }
            .into()
        }
    }
}
