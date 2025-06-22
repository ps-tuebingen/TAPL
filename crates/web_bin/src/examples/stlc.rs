pub const LEFT: &str = include_str!("../../../../examples/stlc/left/left.stlc");

pub const VARIANTCASE: &str = include_str!("../../../../examples/stlc/variantcase/variantcase.stlc");

pub const PROJ: &str = include_str!("../../../../examples/stlc/proj/proj.stlc");

pub const PREDSUCC: &str = include_str!("../../../../examples/stlc/predsucc/predsucc.stlc");

pub const SUMCASE: &str = include_str!("../../../../examples/stlc/sumcase/sumcase.stlc");

pub const IDFUN: &str = include_str!("../../../../examples/stlc/idfun/idfun.stlc");

pub const SND: &str = include_str!("../../../../examples/stlc/snd/snd.stlc");

pub const RECORD: &str = include_str!("../../../../examples/stlc/record/record.stlc");

pub const PAIR: &str = include_str!("../../../../examples/stlc/pair/pair.stlc");

pub const LET: &str = include_str!("../../../../examples/stlc/let/let.stlc");

pub const UNIT: &str = include_str!("../../../../examples/stlc/unit/unit.stlc");

pub const IDNAT: &str = include_str!("../../../../examples/stlc/idnat/idnat.stlc");

pub const LIST: &str = include_str!("../../../../examples/stlc/list/list.stlc");

pub const IF: &str = include_str!("../../../../examples/stlc/if/if.stlc");

pub const SOMECASE: &str = include_str!("../../../../examples/stlc/somecase/somecase.stlc");

pub const SWAP: &str = include_str!("../../../../examples/stlc/swap/swap.stlc");

pub const BOOL: &str = include_str!("../../../../examples/stlc/bool/bool.stlc");

pub const FIX: &str = include_str!("../../../../examples/stlc/fix/fix.stlc");

pub const TUPLE: &str = include_str!("../../../../examples/stlc/tuple/tuple.stlc");

pub const VARIANT: &str = include_str!("../../../../examples/stlc/variant/variant.stlc");

pub const BOOLBOOL: &str = include_str!("../../../../examples/stlc/boolbool/boolbool.stlc");

pub fn stlc_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("Left",LEFT), 
         ("VariantCase",VARIANTCASE), 
         ("Proj",PROJ), 
         ("PredSucc",PREDSUCC), 
         ("SumCase",SUMCASE), 
         ("IdFun",IDFUN), 
         ("Snd",SND), 
         ("Record",RECORD), 
         ("Pair",PAIR), 
         ("Let",LET), 
         ("Unit",UNIT), 
         ("IdNat",IDNAT), 
         ("List",LIST), 
         ("If",IF), 
         ("SomeCase",SOMECASE), 
         ("Swap",SWAP), 
         ("Bool",BOOL), 
         ("Fix",FIX), 
         ("Tuple",TUPLE), 
         ("Variant",VARIANT), 
         ("BoolBool",BOOLBOOL), 

    ]
}
