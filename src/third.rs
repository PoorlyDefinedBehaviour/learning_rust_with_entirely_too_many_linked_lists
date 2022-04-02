use std::rc::Rc;

pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn cons(&self, elem: T) -> List<T> {
    List {
      head: Some(Rc::new(Node {
        elem,
        next: self.head.clone(),
      })),
    }
  }

  pub fn head(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem)
  }

  pub fn tail(&self) -> List<T> {
    List {
      head: self.head.as_ref().and_then(|node| node.next.clone()),
    }
  }

  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter {
      next: self.head.as_deref(),
    }
  }
}

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    let node = self.next?;

    self.next = node.next.as_deref();

    Some(&node.elem)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn smoke() {
    let list = List::new();

    assert_eq!(None, list.head());
    assert_eq!(None, list.tail().head());

    let list = list.cons(1);

    assert_eq!(Some(&1), list.head());

    let list = list.cons(2).cons(3);

    assert_eq!(Some(&3), list.head());

    let list = list.tail();

    assert_eq!(Some(&2), list.head());

    let list = list.tail();

    assert_eq!(Some(&1), list.head());

    let list = list.tail();

    assert_eq!(None, list.head());
  }

  #[test]
  fn iter() {
    assert_eq!(None, List::<i32>::new().iter().next());

    let list = List::new().cons(1).cons(2).cons(3);

    let mut iter = list.iter();

    assert_eq!(Some(&3), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&1), iter.next());
    assert_eq!(None, iter.next());
  }
}
