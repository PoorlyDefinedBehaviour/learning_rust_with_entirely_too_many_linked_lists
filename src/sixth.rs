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

  pub fn peek(&self) -> Option<&T> {
    unsafe { self.head.as_ref().map(|node| &node.elem) }
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    unsafe { self.head.as_mut().map(|node| &mut node.elem) }
  }

  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }

  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter {
      next: if self.head.is_null() {
        None
      } else {
        unsafe { Some(&*self.head) }
      },
    }
  }

  pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
    IterMut {
      next: if self.head.is_null() {
        None
      } else {
        unsafe { Some(&mut *self.head) }
      },
    }
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    while self.pop().is_some() {}
  }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    let node = self.next.take()?;
    self.next = if node.next.is_null() {
      None
    } else {
      unsafe { Some(&*node.next) }
    };
    Some(&node.elem)
  }
}

pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    let node = self.next.take()?;
    self.next = if node.next.is_null() {
      None
    } else {
      unsafe { Some(&mut *node.next) }
    };
    Some(&mut node.elem)
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

  #[test]
  fn sixth_miri_food() {
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);

    assert!(list.pop() == Some(1));
    list.push(4);
    assert!(list.pop() == Some(2));
    list.push(5);

    assert!(list.peek() == Some(&3));
    list.push(6);
    list.peek_mut().map(|x| *x *= 10);
    assert!(list.peek() == Some(&30));
    assert!(list.pop() == Some(30));

    for elem in list.iter_mut() {
      *elem *= 100;
    }

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&400));
    assert_eq!(iter.next(), Some(&500));
    assert_eq!(iter.next(), Some(&600));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    assert!(list.pop() == Some(400));
    list.peek_mut().map(|x| *x *= 10);
    assert!(list.peek() == Some(&5000));
    list.push(7);

    // Drop it on the ground and let the dtor exercise itself
  }
}
