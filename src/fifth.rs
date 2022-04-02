pub struct List<T> {
  head: Link<T>,
  tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> Default for List<T> {
  fn default() -> Self {
    Self {
      head: None,
      tail: std::ptr::null_mut(),
    }
  }
}

impl<T> List<T> {
  pub fn new() -> Self {
    List::default()
  }

  pub fn push(&mut self, elem: T) {
    let mut new_tail = Box::new(Node { elem, next: None });

    let tail_ptr: *mut Node<T> = &mut *new_tail;

    // Are we adding the first element to the list?
    if self.tail.is_null() {
      self.head = Some(new_tail);
    } else {
      unsafe {
        (*self.tail).next = Some(new_tail);
      }
    }

    self.tail = tail_ptr;
  }

  pub fn pop(&mut self) -> Option<T> {
    let mut old_head = self.head.take()?;

    self.head = old_head.next.take();

    // If the list is empty, there's no tail to point to.
    if self.head.is_none() {
      self.tail = std::ptr::null_mut();
    }

    Some(old_head.elem)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn smoke() {
    let mut list = List::new();

    // List is empty, there's nothing to pop.
    assert_eq!(list.pop(), None);

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(Some(1), list.pop());
    assert_eq!(Some(2), list.pop());

    list.push(4);
    list.push(5);

    assert_eq!(Some(3), list.pop());
    assert_eq!(Some(4), list.pop());
    assert_eq!(Some(5), list.pop());
    assert_eq!(None, list.pop());

    list.push(6);
    list.push(7);

    assert_eq!(Some(6), list.pop());
    assert_eq!(Some(7), list.pop());
    assert_eq!(None, list.pop());
  }
}
