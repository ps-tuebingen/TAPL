(\x:Nat.(try 
  (if (iszero(x)) { raise(0) } else { pred(x) }
  catch 
  (\y:Nat.(succ(y))) )))(0)
