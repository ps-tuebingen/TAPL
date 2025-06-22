class List extends Object{
  List() { super(); } 

  Int head() { return 0; }

  List tail() { return new List(); }
}

class Nil extends List { 
  Nil() { super(); }
}

class Cons extends List{
  Int first;
  List rest;

  Cons(Int first,List rest) {
    super(); 
    this.first = first;
    this.rest = rest;
  }

  Int head() {
    return this.first;
  }

  List tail(){
    return this.rest;
  }
}
