class Node<T> {

  public next: Node<T> | null = null;

  constructor(
    public val: T,
  ) { }

}


export class Queue<T> {


  private first: Node<T> | null;
  private last: Node<T> | null;
  private size: number;

  constructor() {
    this.first = null;
    this.last = null;
    this.size = 0;
  }

  public get_size(): number {
    return this.size;
  }

  public push(val: T): void {
    this.size++;
    if (this.first === null) {
      this.first = new Node(val);
      this.last = this.first;
    } else {
      this.last.next = new Node(val);
      this.last = this.last.next;
    }
  }

  public peek(): T | null {
    return this.first?.val;
  }

  // problematic negative size if returning null. but idc.
  //  Why doesnt JS have a standard lib with datastructures like every other good lang?
  //  I blame js
  //  (EZ fix if you want to become a contributor:)
  public pop(): T | null {
    let val = this.first?.val;
    this.first = this.first?.next;
    this.size--;
    return val;
  }

}
