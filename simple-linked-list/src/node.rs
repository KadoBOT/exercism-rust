pub struct Node<T> {
  pub data: T,
  pub next: Option<Box<Node<T>>>,
}

impl<T: Copy> Node<T> {
  pub fn new(element: T, next: Option<Box<Node<T>>>) -> Self {
    Node {
      data: element,
      next,
    }
  }
}
