pub const ISZZ: &str = include_str!("../../../../examples/typed_arithmetic/iszz/iszz.arith");

pub const IF_NESTED: &str = include_str!("../../../../examples/typed_arithmetic/if_nested/if_nested.arith");

pub const IFTHENELSE: &str = include_str!("../../../../examples/typed_arithmetic/ifthenelse/ifthenelse.arith");

pub fn typed_arithmetic_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("IsZeroZero",ISZZ), 
         ("IfNested",IF_NESTED), 
         ("If",IFTHENELSE), 

    ]
}
