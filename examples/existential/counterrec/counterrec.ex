{*{x:Nat}, {
  new = {x=1},
  get = \l:{x:Nat}.i.x,
  inc = \l.{x:Nat}. {x=succ(i.x)}
  }
} as {exists Counter, {
  new: Counter,
  get: Counter -> Nat,
  inc: Counter -> Counter
  }
}
