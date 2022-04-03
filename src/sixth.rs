pub struct List<T> {
  head: *mut Node<T>,
  tail: *mut Node<T>,
}

struct Node<T> {
  elem: T,
  next: *mut Node<T>,
}

impl<T> Default for List<T> {
  fn default() -> Self {
    Self {
      head: std::ptr::null_mut(),
      tail: std::ptr::null_mut(),
    }
  }
}

impl<T> List<T> {
  pub fn new() -> Self {
    List::default()
  }

  pub fn push(&mut self, elem: T) {
    unsafe {
      let new_tail = Box::into_raw(Box::new(Node {
        elem,
        next: std::ptr::null_mut(),
      }));

      // If this is the first element that's being added to the list.
      if self.tail.is_null() {
        // The new element becomes the head.
        self.head = new_tail;
      } else {
        // Add element to the end of the list.
        (*self.tail).next = new_tail;
      }

      // Point to the last element in the list.
      self.tail = new_tail;
    }
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.head.is_null() {
      None
    } else {
      unsafe {
        let old_head = Box::from_raw(self.head);

        self.head = old_head.next;

        // If list became empty, there's nothing for the the tail to point to.
        if self.head.is_null() {
          self.tail = std::ptr::null_mut();
        }

        Some(old_head.elem)
      }
    }
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    while self.pop().is_some() {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sixth_smoke() {
    let mut list = List::new();

    assert_eq!(None, list.pop());

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
