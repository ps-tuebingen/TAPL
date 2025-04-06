{*Nat, {
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
}}
