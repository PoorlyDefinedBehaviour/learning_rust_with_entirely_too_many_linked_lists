use std::{
  cell::{Ref, RefCell, RefMut},
  rc::Rc,
};

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
  #[inline]
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

  pub fn peek_front(&self) -> Option<Ref<T>> {
    self
      .head
      .as_ref()
      .map(|node| Ref::map(node.borrow(), |node| &node.elem))
  }

  pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
    self
      .head
      .as_ref()
      .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
  }

  pub fn push_back(&mut self, elem: T) {
    let new_tail = Node::new(elem);

    match self.tail.take() {
      Some(old_tail) => {
        old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
        new_tail.borrow_mut().prev = Some(old_tail);
        self.tail = Some(new_tail);
      }
      None => {
        self.head = Some(Rc::clone(&new_tail));
        self.tail = Some(new_tail);
      }
    }
  }

  pub fn pop_back(&mut self) -> Option<T> {
    let old_tail = self.tail.take()?;

    match old_tail.borrow_mut().prev.take() {
      Some(new_tail) => {
        new_tail.borrow_mut().next.take();
        self.tail = Some(new_tail);
      }
      None => {
        self.head.take();
      }
    }

    let node = Rc::try_unwrap(old_tail).ok().unwrap().into_inner();

    Some(node.elem)
  }

  pub fn peek_back(&self) -> Option<Ref<T>> {
    self
      .tail
      .as_ref()
      .map(|node| Ref::map(node.borrow(), |node| &node.elem))
  }

  pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
    self
      .tail
      .as_ref()
      .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
  }

  #[inline]
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    while self.pop_front().is_some() {}
  }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop_front()
  }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
  #[inline]
  fn next_back(&mut self) -> Option<Self::Item> {
    self.0.pop_back()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn push_front_and_pop_front() {
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

  #[test]
  fn push_front_and_peek_front() {
    let mut list = List::new();

    assert!(matches!(list.peek_front(), None));

    list.push_front(1);

    assert_eq!(1, *list.peek_front().unwrap());

    list.push_front(2);

    assert_eq!(2, *list.peek_front().unwrap());

    list.push_front(3);

    assert_eq!(3, *list.peek_front().unwrap());
  }

  #[test]
  fn into_iter_iterator() {
    let mut list = List::new();

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    let mut iter = list.into_iter();

    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(1), iter.next());
  }

  #[test]
  fn into_iter_double_ended_iterator() {
    let mut list = List::new();

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    let mut iter = list.into_iter();

    assert_eq!(Some(1), iter.next_back());
    assert_eq!(Some(2), iter.next_back());
    assert_eq!(Some(3), iter.next_back());
  }

  // NOTE: should test more permutations. Not going to because i can't be bothered.
}
