pub const C2: &str = include_str!("../../../../examples/system_f/c2/c2.f");

pub const NOT: &str = include_str!("../../../../examples/system_f/not/not.f");

pub const CTIMES: &str = include_str!("../../../../examples/system_f/ctimes/ctimes.f");

pub const C0: &str = include_str!("../../../../examples/system_f/c0/c0.f");

pub const ID: &str = include_str!("../../../../examples/system_f/id/id.f");

pub const TRU: &str = include_str!("../../../../examples/system_f/tru/tru.f");

pub const NIL: &str = include_str!("../../../../examples/system_f/nil/nil.f");

pub const CONS: &str = include_str!("../../../../examples/system_f/cons/cons.f");

pub const CEXP: &str = include_str!("../../../../examples/system_f/cexp/cexp.f");

pub const DOUBLE: &str = include_str!("../../../../examples/system_f/double/double.f");

pub const ISNIL: &str = include_str!("../../../../examples/system_f/isnil/isnil.f");

pub const CSUCC: &str = include_str!("../../../../examples/system_f/csucc/csucc.f");

pub const QUADRUPLE: &str = include_str!("../../../../examples/system_f/quadruple/quadruple.f");

pub const SELFAPP: &str = include_str!("../../../../examples/system_f/selfapp/selfapp.f");

pub const CPLUS: &str = include_str!("../../../../examples/system_f/cplus/cplus.f");

pub const C1: &str = include_str!("../../../../examples/system_f/c1/c1.f");

pub const FLS: &str = include_str!("../../../../examples/system_f/fls/fls.f");

pub fn system_f_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("c2",C2), 
         ("not",NOT), 
         ("ctimes",CTIMES), 
         ("c0",C0), 
         ("id",ID), 
         ("tru",TRU), 
         ("nil",NIL), 
         ("cons",CONS), 
         ("cexp",CEXP), 
         ("double",DOUBLE), 
         ("isnil",ISNIL), 
         ("csucc",CSUCC), 
         ("quadruple",QUADRUPLE), 
         ("selfapp",SELFAPP), 
         ("cplus",CPLUS), 
         ("c1",C1), 
         ("fls",FLS), 

    ]
}
