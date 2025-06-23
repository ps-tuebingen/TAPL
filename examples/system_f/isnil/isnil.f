def main := 
  \X.\l:(forall R.(X->R->R)->R->R).
  ((l[forall X.((X -> (X -> X)))])(\hd:X.\tl:(forall X.((X -> (X -> X)))).(\X.\t:X.\f:X.f)))(\X.\t:X.\f:X.t);
