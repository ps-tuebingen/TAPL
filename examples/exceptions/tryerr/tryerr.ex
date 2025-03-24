(\x:Nat.(try 
  (if (iszero(x)) { error } else { pred(x) })
  with 0))(0)

