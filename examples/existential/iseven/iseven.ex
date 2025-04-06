fix(
  \f:Nat->Nat.\n:Nat. 
    if (iszero(n)) {
      true
    }else {
      if (iszero(pred(n))){
        false
      }else{
        f(pred(n))
      }
    }
)
