pub const VARIANTCASE: &str = include_str!("../../../../examples/stlc/variantcase/variantcase.stlc");

pub const BOOLBOOL: &str = include_str!("../../../../examples/stlc/boolbool/boolbool.stlc");

pub const IDNAT: &str = include_str!("../../../../examples/stlc/idnat/idnat.stlc");

pub const SOMECASE: &str = include_str!("../../../../examples/stlc/somecase/somecase.stlc");

pub const LET: &str = include_str!("../../../../examples/stlc/let/let.stlc");

pub const PAIR: &str = include_str!("../../../../examples/stlc/pair/pair.stlc");

pub const LEFT: &str = include_str!("../../../../examples/stlc/left/left.stlc");

pub const VARIANT: &str = include_str!("../../../../examples/stlc/variant/variant.stlc");

pub const SUMCASE: &str = include_str!("../../../../examples/stlc/sumcase/sumcase.stlc");

pub const PROJ: &str = include_str!("../../../../examples/stlc/proj/proj.stlc");

pub const FIX: &str = include_str!("../../../../examples/stlc/fix/fix.stlc");

pub const BOOL: &str = include_str!("../../../../examples/stlc/bool/bool.stlc");

pub const PREDSUCC: &str = include_str!("../../../../examples/stlc/predsucc/predsucc.stlc");

pub const SND: &str = include_str!("../../../../examples/stlc/snd/snd.stlc");

pub const TUPLE: &str = include_str!("../../../../examples/stlc/tuple/tuple.stlc");

pub const UNIT: &str = include_str!("../../../../examples/stlc/unit/unit.stlc");

pub const SWAP: &str = include_str!("../../../../examples/stlc/swap/swap.stlc");

pub const LIST: &str = include_str!("../../../../examples/stlc/list/list.stlc");

pub const RECORD: &str = include_str!("../../../../examples/stlc/record/record.stlc");

pub const IDFUN: &str = include_str!("../../../../examples/stlc/idfun/idfun.stlc");

pub const IF: &str = include_str!("../../../../examples/stlc/if/if.stlc");

pub fn stlc_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("VariantCase",VARIANTCASE), 
         ("BoolBool",BOOLBOOL), 
         ("IdNat",IDNAT), 
         ("SomeCase",SOMECASE), 
         ("Let",LET), 
         ("Pair",PAIR), 
         ("Left",LEFT), 
         ("Variant",VARIANT), 
         ("SumCase",SUMCASE), 
         ("Proj",PROJ), 
         ("Fix",FIX), 
         ("Bool",BOOL), 
         ("PredSucc",PREDSUCC), 
         ("Snd",SND), 
         ("Tuple",TUPLE), 
         ("Unit",UNIT), 
         ("Swap",SWAP), 
         ("List",LIST), 
         ("Record",RECORD), 
         ("IdFun",IDFUN), 
         ("If",IF), 

    ]
}
