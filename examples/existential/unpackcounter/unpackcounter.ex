let {Counter, counter} = 
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
  } in 
(counter.get)((counter.inc) (counter.new))
