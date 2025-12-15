//Automatically generated file, run `cargo run -p xtask` to regenerate
pub const BOOL: &str = include_str!("../../../../examples/stlc/bool/bool.stlc");

pub const IF: &str = include_str!("../../../../examples/stlc/if/if.stlc");

pub const TUPLE: &str = include_str!("../../../../examples/stlc/tuple/tuple.stlc");

pub const FIX: &str = include_str!("../../../../examples/stlc/fix/fix.stlc");

pub const SUMCASE: &str = include_str!("../../../../examples/stlc/sumcase/sumcase.stlc");

pub const VARIANT: &str = include_str!("../../../../examples/stlc/variant/variant.stlc");

pub const IDNAT: &str = include_str!("../../../../examples/stlc/idnat/idnat.stlc");

pub const BOOLBOOL: &str = include_str!("../../../../examples/stlc/boolbool/boolbool.stlc");

pub const SND: &str = include_str!("../../../../examples/stlc/snd/snd.stlc");

pub const PREDSUCC: &str = include_str!("../../../../examples/stlc/predsucc/predsucc.stlc");

pub const LET: &str = include_str!("../../../../examples/stlc/let/let.stlc");

pub const LEFT: &str = include_str!("../../../../examples/stlc/left/left.stlc");

pub const UNIT: &str = include_str!("../../../../examples/stlc/unit/unit.stlc");

pub const SOMECASE: &str = include_str!("../../../../examples/stlc/somecase/somecase.stlc");

pub const VARIANTCASE: &str = include_str!("../../../../examples/stlc/variantcase/variantcase.stlc");

pub const LIST: &str = include_str!("../../../../examples/stlc/list/list.stlc");

pub const PROJ: &str = include_str!("../../../../examples/stlc/proj/proj.stlc");

pub const IDFUN: &str = include_str!("../../../../examples/stlc/idfun/idfun.stlc");

pub const RECORD: &str = include_str!("../../../../examples/stlc/record/record.stlc");

pub const SWAP: &str = include_str!("../../../../examples/stlc/swap/swap.stlc");

pub const PAIR: &str = include_str!("../../../../examples/stlc/pair/pair.stlc");

pub fn stlc_all() -> Vec<(&'static str,&'static str)> {
    vec![
        ("Bool", BOOL),
        ("If", IF),
        ("Tuple", TUPLE),
        ("Fix", FIX),
        ("Sumcase", SUMCASE),
        ("Variant", VARIANT),
        ("Idnat", IDNAT),
        ("Boolbool", BOOLBOOL),
        ("Snd", SND),
        ("Predsucc", PREDSUCC),
        ("Let", LET),
        ("Left", LEFT),
        ("Unit", UNIT),
        ("Somecase", SOMECASE),
        ("Variantcase", VARIANTCASE),
        ("List", LIST),
        ("Proj", PROJ),
        ("Idfun", IDFUN),
        ("Record", RECORD),
        ("Swap", SWAP),
        ("Pair", PAIR),
    ]
}
