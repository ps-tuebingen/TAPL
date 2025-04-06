\c:Counter.
  (\c:{exists X, {
    state:X,
    methods:{
      get:X->Nat,
      inc:X->X
      }  
    }}.
    let {X,body} = c in {*X, {
      state = ((body.methods).inc)(body.state),
      methods = body.methds}
    } as {exists X, {
      state:X,
      methods:{
        get:X->Nat,
        inc:X->X
      }  
    }}
  )(
  (\c:{exists X, {
    state:X,
    methods:{
      get:X->Nat,
      inc:X->X
    }  
    }}.
    let {X,body} = c in {*X, {
      state = ((body.methods).inc)(body.state),
      methods = body.methds}
    } as {exists X, {
      state:X,
      methods:{
        get:X->Nat,
        inc:X->X
      }  
    }})
  )(
  (\c:{exists X, {
      state:X,
      methods:{
        get:X->Nat,
        inc:X->X
      }  
    }}.
    let {X,body} = c in {*X, {
      state = ((body.methods).inc)(body.state),
      methods = body.methds}
    } as {exists X, { 
      state:X,
      methods:{
        get:X->Nat,
        inc:X->X
      }  
    }}
)(c))
