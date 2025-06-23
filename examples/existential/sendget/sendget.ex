def main := 
  \c:{exists X, {
    state:X,
    methods:{
      get:X->Nat,
      inc:X->X
    }  
  }}. let {X,body} = c in 
  (((body.methods).get)(body.state));
