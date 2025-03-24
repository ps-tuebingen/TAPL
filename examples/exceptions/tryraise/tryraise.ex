(\x:Nat.(
  try {
    if (iszero(x)) { raise[Nat](0:Nat) } else { pred(x) }
  } catch {
    (\y:Nat.(succ(y))) 
  }
))(0)
