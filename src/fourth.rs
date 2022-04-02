use std::{cell::RefCell, rc::Rc};

pub struct List<T> {
  head: Link<T>,
  tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
  prev: Link<T>,
}

impl<T> Node<T> {
  pub fn new(elem: T) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Node {
      elem,
      prev: None,
      next: None,
    }))
  }
}

impl<T> Default for List<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> List<T> {
  pub fn new() -> Self {
    Self {
      head: None,
      tail: None,
    }
  }

  pub fn push_front(&mut self, elem: T) {
    let new_head = Node::new(elem);

    match self.head.take() {
      Some(old_head) => {
        old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
        new_head.borrow_mut().next = Some(old_head);
        self.head = Some(new_head);
      }
      None => {
        self.tail = Some(Rc::clone(&new_head));
        self.head = Some(new_head);
      }
    }
  }

  pub fn pop_front(&mut self) -> Option<T> {
    let head = self.head.take()?;

    // Does the list have more than one element?
    match head.borrow_mut().next.take() {
      Some(new_head) => {
        let _ = new_head.borrow_mut().prev.take();
        self.head = Some(Rc::clone(&new_head));
        self.tail = self.head.clone();
      }
      None => {
        // List is empty since we removed the only element it had.
        let _ = self.tail.take();
      }
    }

    let node = Rc::try_unwrap(head).ok().unwrap().into_inner();

    Some(node.elem)
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    while self.pop_front().is_some() {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn smoke() {
    let mut list = List::new();

    assert_eq!(None, list.pop_front());

    list.push_front(1);

    assert_eq!(Some(1), list.pop_front());

    list.push_front(2);
    list.push_front(3);

    assert_eq!(Some(3), list.pop_front());
    assert_eq!(Some(2), list.pop_front());
    assert_eq!(None, list.pop_front());
  }
}
