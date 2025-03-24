class List extends Object{
  List() { super(); } 

  int head() { return 0; }

  List tail() { return new List() };
}

class Nil extends List { 
  Nil() { super(); }
}

class Cons extends List{
  int first;
  List rest;

  Cons(int first,List rest) {
    super(); 
    self.first = first;
    self.rest= rest;
  }

  int head() {
    return self.first
  }

  List tail(){
    return sefl.rest
  }
}
