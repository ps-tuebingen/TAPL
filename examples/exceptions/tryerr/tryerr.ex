(\x:Nat.(try 
  (if (iszero(x)) { error[Nat] } else { pred(x) })
  with 0))(0)

