/// Because List if a struct with a single field, its size is the same as the field.
pub struct List {
  head: Link,
}

enum Link {
  Empty,
  More(Box<Node>),
}

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    Self { head: Link::Empty }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Node {
      elem,
      next: std::mem::replace(&mut self.head, Link::Empty),
    };

    self.head = Link::More(Box::new(new_node));
  }

  pub fn pop(&mut self) -> Option<i32> {
    match std::mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => None,
      Link::More(node) => {
        let result = Some(node.elem);
        self.head = node.next;
        result
      }
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
}
