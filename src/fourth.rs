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
}