def object :: {exists X, {
  state:X,
  methods:{
    get:X->Nat,
    inc:X->X
  }  
}} := {*Nat, {
  state = 5,
  methods = { 
    get = \x:Nat.x,
    inc = \x:Nat.succ(x)
    }
  }
} as {exists X, {
  state:X,
  methods:{
    get:X->Nat,
    inc:X->X
  }  
}};

def main := let {X,obj} = object in 
  (((obj.methods).get) (((obj.methods).inc) (obj.state)));
