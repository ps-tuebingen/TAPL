def cnt ::{exists Counter, 
    { new: Counter,
      get: Counter->Nat,
      inc: Counter->Counter
    }
  } := {*Nat, 
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

def flp ::{exists FlipFlop,
      { new: FlipFlop,
        read: FlipFlop->Bool,
        toggle: FlipFlop->FlipFlop,
        reset: FlipFlop->FlipFlop
      }
    } := let {Counter,counter} = cnt in 
    {*Counter,
      { new=counter.new,
        read=\c:Counter.(fix(
            \f:Nat->Bool.\n:Nat. 
            if (iszero(n)) {
              true
            }else {
              if (iszero(pred(n))){
                false
              }else{
                f(pred(n))
              }
            }))((counter.get)(c)),
        toggle=\c:Counter. ((counter.inc) c),
        reset=\c:Counter.(counter.new)
        }
    } as {exists FlipFlop,
      { new: FlipFlop,
        read: FlipFlop->Bool,
        toggle: FlipFlop->FlipFlop,
        reset: FlipFlop->FlipFlop
      }
    };

def main := let {FlipFlop,flipflop} = flp in (flipflop.read)((flipflop.toggle)((flipflop.toggle)(flipflop.new)));
