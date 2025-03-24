(\x:Nat.(try 
  (if (iszero(x)) { raise(0) } else { pred(x) }
  with 
  (\y:Nat.(succ(y))) )))(0)
