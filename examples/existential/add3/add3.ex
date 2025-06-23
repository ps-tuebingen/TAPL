def inc :: {exists X, 
    { state:X,
      methods:
      { get:X->Nat,
        inc:X->X
      }  
    }
  } -> {exists X, { 
      state:X,
      methods:{
        get:X->Nat,
        inc:X->X
      }  
    }} := 
  \c:{exists X, 
    { state:X,
      methods:
      { get:X->Nat,
        inc:X->X
      }  
    }
  }.
  let {X,body} = c in {*X, 
    {
      state = ((body.methods).inc)(body.state),
      methods = body.methods
    } } as {exists X, { 
      state:X,
      methods:{
        get:X->Nat,
        inc:X->X
      }  
    }};

def main := \c:{exists X, 
  {
    state:X,
    methods:{
      get:X->Nat,
      inc:X->X
      }  
  }}.
  ((inc) (inc ((inc) (c))));
