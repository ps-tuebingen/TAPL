def rec :: {exists Counter, {
  new: Counter,
  get: Counter -> Nat,
  inc: Counter -> Counter
  }
} := {*{x:Nat}, {
  new = {x=1},
  get = \l:{x:Nat}.l.x,
  inc = \l:{x:Nat}. {x=succ(l.x)}
  }
} as {exists Counter, {
  new: Counter,
  get: Counter -> Nat,
  inc: Counter -> Counter
  }
};

def main := let {Counter,counter} = rec in ((counter.get) counter.new);
