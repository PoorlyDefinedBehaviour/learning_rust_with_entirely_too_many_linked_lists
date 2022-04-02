/// Because List if a struct with a single field, its size is the same as the field.
pub struct List {
  head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    Self { head: None }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Node {
      elem,
      next: self.head.take(),
    };

    self.head = Some(Box::new(new_node));
  }

  pub fn pop(&mut self) -> Option<i32> {
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
}

impl Drop for List {
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
}
