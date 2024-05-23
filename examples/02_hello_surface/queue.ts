class Node<T> {

  public next: Node<T> | null = null;

  constructor(
    public val: T,
  ) { }

}


export class Queue<T> {

  private first: Node<T> | null;
  private last: Node<T> | null;

  constructor() {
    this.first = null;
    this.last = null;
  }

  public push(val: T): void {
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

  public pop(): T | null {
    let val = this.first?.val;
    this.first = this.first?.next;
    return val;
  }

}
