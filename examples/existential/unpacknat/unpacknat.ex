let {X,x} = {*Nat, {a=0, f=\x:Nat.succ(x)}} as {exists X, {a:X,f:X->Nat}} in (x.f)(x.a)
