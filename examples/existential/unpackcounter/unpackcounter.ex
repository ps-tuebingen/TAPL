def cnt :: {exists Counter, 
    { new: Counter,
      get: Counter->Nat,
      inc: Counter->Counter
    }
  } :=
  {*Nat, 
    { new=1,
      get=\i:Nat.i,
      inc=\i:Nat. succ(i)
    }
  } as {exists Counter, 
    { new: Counter,
      get: Counter->Nat,
      inc: Counter->Counter
    }
  };

def main := let {Counter, counter} = cnt in 
  (counter.get)((counter.inc) (counter.new));
