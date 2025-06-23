def nat :: {exists X, {a:X,f:X->Nat}} := 
  {*Nat, {a=0, f=\x:Nat.succ(x)}} as {exists X, {a:X,f:X->Nat}};

def main := let {X,x} = nat in (x.f)(x.a);
