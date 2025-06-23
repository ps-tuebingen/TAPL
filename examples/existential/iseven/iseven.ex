def ise :: Nat -> Bool := fix(
  \f:Nat->Bool.\n:Nat. 
    if (iszero(n)) {
      true
    }else {
      if (iszero(pred(n))){
        false
      }else{
        f(pred(n))
      }
    }
);

def main := ise 5;
