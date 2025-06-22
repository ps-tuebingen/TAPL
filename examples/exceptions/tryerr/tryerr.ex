def predz :: Nat -> Nat := 
  \x:Nat.try {
         if (iszero(x)) { error[Nat] } else { pred(x) }
  } with {
    0
  };

def main := (predz)(0);
