def main := 
  {*Bool, {a=true, f=\x:Bool.if (x) { false } else { true } }}
  as {exists X, {a:X, f:X->X}};
