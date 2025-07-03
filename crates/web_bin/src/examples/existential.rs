pub const SENDINC: &str = include_str!("../../../../examples/existential/sendinc/sendinc.ex");

pub const COUNTERADT: &str = include_str!("../../../../examples/existential/counteradt/counteradt.ex");

pub const UNPACKCOUNTER: &str = include_str!("../../../../examples/existential/unpackcounter/unpackcounter.ex");

pub const ADD3: &str = include_str!("../../../../examples/existential/add3/add3.ex");

pub const UNPACKNAT: &str = include_str!("../../../../examples/existential/unpacknat/unpacknat.ex");

pub const COUNTEROBJECT: &str = include_str!("../../../../examples/existential/counterobject/counterobject.ex");

pub const PACKBOOL: &str = include_str!("../../../../examples/existential/packbool/packbool.ex");

pub const SENDGET: &str = include_str!("../../../../examples/existential/sendget/sendget.ex");

pub const PACKNAT: &str = include_str!("../../../../examples/existential/packnat/packnat.ex");

pub const ISEVEN: &str = include_str!("../../../../examples/existential/iseven/iseven.ex");

pub const FLIPFLOP: &str = include_str!("../../../../examples/existential/flipflop/flipflop.ex");

pub const COUNTERREC: &str = include_str!("../../../../examples/existential/counterrec/counterrec.ex");

pub fn existential_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("SendInc",SENDINC), 
         ("CounterADT",COUNTERADT), 
         ("UnpackCounter",UNPACKCOUNTER), 
         ("Add3",ADD3), 
         ("UnpackNat",UNPACKNAT), 
         ("CounterObject",COUNTEROBJECT), 
         ("PackBool",PACKBOOL), 
         ("SendGet",SENDGET), 
         ("PackNat",PACKNAT), 
         ("IsEven",ISEVEN), 
         ("FlipFlop",FLIPFLOP), 
         ("CounterRec",COUNTERREC), 

    ]
}
