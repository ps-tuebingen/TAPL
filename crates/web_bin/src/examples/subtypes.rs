pub const COUNTERCLASS: &str = include_str!("../../../../examples/subtypes/counterclass/counterclass.sub");

pub const VARIANTCASE: &str = include_str!("../../../../examples/subtypes/variantcase/variantcase.sub");

pub const NEWRESETCOUNTER: &str = include_str!("../../../../examples/subtypes/newresetcounter/newresetcounter.sub");

pub const LAMBDAREC: &str = include_str!("../../../../examples/subtypes/lambdarec/lambdarec.sub");

pub const INSTRCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/instrcounterclass/instrcounterclass.sub");

pub const DOWNCAST: &str = include_str!("../../../../examples/subtypes/downcast/downcast.sub");

pub const REFNAT: &str = include_str!("../../../../examples/subtypes/refnat/refnat.sub");

pub const SETCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/setcounterclass/setcounterclass.sub");

pub const LIST: &str = include_str!("../../../../examples/subtypes/list/list.sub");

pub const BACKUPCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/backupcounterclass/backupcounterclass.sub");

pub const COUNTER: &str = include_str!("../../../../examples/subtypes/counter/counter.sub");

pub const NEWCOUNTER: &str = include_str!("../../../../examples/subtypes/newcounter/newcounter.sub");

pub const NEWDECCOUNTER: &str = include_str!("../../../../examples/subtypes/newdeccounter/newdeccounter.sub");

pub const DECCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/deccounterclass/deccounterclass.sub");

pub const NEWSETCOUNTER: &str = include_str!("../../../../examples/subtypes/newsetcounter/newsetcounter.sub");

pub const RESETCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/resetcounterclass/resetcounterclass.sub");

pub const VARIANT: &str = include_str!("../../../../examples/subtypes/variant/variant.sub");

pub fn subtypes_all() -> Vec<(&'static str,&'static str)> { 
    vec![
        ("l]]]]w]l][]]",COUNTERCLASS), 
("][]][]]l[]w",VARIANTCASE), 
("]w]]w]w]l]]]]w]",NEWRESETCOUNTER), 
("][]:o[]wl",LAMBDAREC), 
("]]]]]l]]]]w]l][]]",INSTRCOUNTERCLASS), 
("o]]]l[]]",DOWNCAST), 
("]we][]",REFNAT), 
("]w]l]]]]w]l][]]",SETCOUNTERCLASS), 
("]]]]",LIST), 
(":[l]]]l]]]]w]l][]]",BACKUPCOUNTERCLASS), 
("l]]]]w]",COUNTER), 
("]w]l]]]]w]",NEWCOUNTER), 
("]w]owll]]]]w]",NEWDECCOUNTER), 
("owll]]]]w]l][]]",DECCOUNTERCLASS), 
("]w]]w]l]]]]w]",NEWSETCOUNTER), 
("]w]w]l]]]]w]l][]]",RESETCOUNTERCLASS), 
("][]][]]",VARIANT), 

    ]
}
