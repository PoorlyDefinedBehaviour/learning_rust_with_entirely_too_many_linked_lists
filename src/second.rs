/// Because List if a struct with a single field, its size is the same as the field.
pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    Self { head: None }
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Node {
      elem,
      next: self.head.take(),
    };

    self.head = Some(Box::new(new_node));
  }

  pub fn pop(&mut self) -> Option<T> {
    // NOTE: the author suggests we use Option::map here but i think
    // Option::map should be pure and we mutate self.head.
    match self.head.take() {
      None => None,
      Some(node) => {
        let result = Some(node.elem);
        self.head = node.next;
        result
      }
    }
  }

  pub fn peek(&self) -> Option<&T> {
    self.head.as_deref().map(|node| &node.elem)
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| &mut node.elem)
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();

    while let Some(mut boxed_node) = cur_link {
      cur_link = boxed_node.next.take();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn smoke() {
    let mut list = List::new();

    assert_eq!(None, list.pop());

    list.push(1);

    assert_eq!(Some(1), list.pop());

    assert_eq!(None, list.pop());

    list.push(3);
    list.push(2);
    list.push(1);

    assert_eq!(Some(1), list.pop());
    assert_eq!(Some(2), list.pop());
    assert_eq!(Some(3), list.pop());
    assert_eq!(None, list.pop());
  }

  #[test]
  fn peek() {
    let mut list = List::new();

    assert_eq!(None, list.peek());

    list.push(1);

    assert_eq!(Some(&1), list.peek());

    list.push(2);

    assert_eq!(Some(&2), list.peek());
  }

  #[test]
  fn peek_mut() {
    let mut list = List::new();

    assert_eq!(None, list.peek_mut());

    list.push(1);

    assert_eq!(Some(&mut 1), list.peek_mut());

    list.push(2);

    let value = list.peek_mut();

    assert_eq!(Some(&mut 2), value);

    *value.unwrap() = 10;

    assert_eq!(Some(&mut 10), list.peek_mut());
  }
}
