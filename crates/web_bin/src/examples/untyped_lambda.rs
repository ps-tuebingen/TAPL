pub const TIMES: &str = include_str!("../../../../examples/untyped_lambda/times/times.lam");

pub const FALSE: &str = include_str!("../../../../examples/untyped_lambda/false/false.lam");

pub const ONE: &str = include_str!("../../../../examples/untyped_lambda/one/one.lam");

pub const ID: &str = include_str!("../../../../examples/untyped_lambda/id/id.lam");

pub const SND: &str = include_str!("../../../../examples/untyped_lambda/snd/snd.lam");

pub const TRU: &str = include_str!("../../../../examples/untyped_lambda/tru/tru.lam");

pub const PAIR: &str = include_str!("../../../../examples/untyped_lambda/pair/pair.lam");

pub const FST: &str = include_str!("../../../../examples/untyped_lambda/fst/fst.lam");

pub const IFTHENELSE: &str = include_str!("../../../../examples/untyped_lambda/ifthenelse/ifthenelse.lam");

pub const ZERO: &str = include_str!("../../../../examples/untyped_lambda/zero/zero.lam");

pub const ISZERO: &str = include_str!("../../../../examples/untyped_lambda/iszero/iszero.lam");

pub const SUCC: &str = include_str!("../../../../examples/untyped_lambda/succ/succ.lam");

pub const AND: &str = include_str!("../../../../examples/untyped_lambda/and/and.lam");

pub const PLUS: &str = include_str!("../../../../examples/untyped_lambda/plus/plus.lam");

pub const PRD: &str = include_str!("../../../../examples/untyped_lambda/prd/prd.lam");

pub fn untyped_lambda_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("times",TIMES), 
         ("false",FALSE), 
         ("one",ONE), 
         ("id",ID), 
         ("snd",SND), 
         ("tru",TRU), 
         ("pair",PAIR), 
         ("fst",FST), 
         ("ifthenelse",IFTHENELSE), 
         ("zero",ZERO), 
         ("iszero",ISZERO), 
         ("succ",SUCC), 
         ("and",AND), 
         ("plus",PLUS), 
         ("prd",PRD), 

    ]
}
