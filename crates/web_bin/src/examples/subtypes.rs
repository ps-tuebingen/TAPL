pub const RESETCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/resetcounterclass/resetcounterclass.sub");

pub const VARIANTCASE: &str = include_str!("../../../../examples/subtypes/variantcase/variantcase.sub");

pub const NEWCOUNTER: &str = include_str!("../../../../examples/subtypes/newcounter/newcounter.sub");

pub const REFNAT: &str = include_str!("../../../../examples/subtypes/refnat/refnat.sub");

pub const DOWNCAST: &str = include_str!("../../../../examples/subtypes/downcast/downcast.sub");

pub const BACKUPCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/backupcounterclass/backupcounterclass.sub");

pub const COUNTER: &str = include_str!("../../../../examples/subtypes/counter/counter.sub");

pub const NEWRESETCOUNTER: &str = include_str!("../../../../examples/subtypes/newresetcounter/newresetcounter.sub");

pub const INSTRCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/instrcounterclass/instrcounterclass.sub");

pub const VARIANT: &str = include_str!("../../../../examples/subtypes/variant/variant.sub");

pub const DECCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/deccounterclass/deccounterclass.sub");

pub const SETCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/setcounterclass/setcounterclass.sub");

pub const NEWSETCOUNTER: &str = include_str!("../../../../examples/subtypes/newsetcounter/newsetcounter.sub");

pub const LAMBDAREC: &str = include_str!("../../../../examples/subtypes/lambdarec/lambdarec.sub");

pub const NEWDECCOUNTER: &str = include_str!("../../../../examples/subtypes/newdeccounter/newdeccounter.sub");

pub const COUNTERCLASS: &str = include_str!("../../../../examples/subtypes/counterclass/counterclass.sub");

pub const LIST: &str = include_str!("../../../../examples/subtypes/list/list.sub");

pub fn subtypes_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("ResetCounterClass",RESETCOUNTERCLASS), 
         ("VariantCase",VARIANTCASE), 
         ("NewCounter",NEWCOUNTER), 
         ("RefNat",REFNAT), 
         ("DownCast",DOWNCAST), 
         ("BackupCounterClass",BACKUPCOUNTERCLASS), 
         ("Counter",COUNTER), 
         ("NewResetCounter",NEWRESETCOUNTER), 
         ("InstrCounterClass",INSTRCOUNTERCLASS), 
         ("Variant",VARIANT), 
         ("DecCounterClass",DECCOUNTERCLASS), 
         ("SetCounterClass",SETCOUNTERCLASS), 
         ("NewSetCounter",NEWSETCOUNTER), 
         ("LambdaRec",LAMBDAREC), 
         ("NewDecCounter",NEWDECCOUNTER), 
         ("CounterClass",COUNTERCLASS), 
         ("List",LIST), 

    ]
}
