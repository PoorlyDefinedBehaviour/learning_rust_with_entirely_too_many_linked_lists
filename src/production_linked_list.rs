use std::{
  boxed,
  cmp::Ordering,
  fmt::{self, Debug},
  hash::{Hash, Hasher},
  marker::PhantomData,
  ptr::NonNull,
};

pub struct LinkedList<T> {
  front: Link<T>,
  back: Link<T>,
  len: usize,
  _p: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
  front: Link<T>,
  back: Link<T>,
  elem: T,
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    Self {
      front: None,
      back: None,
      len: 0,
      _p: PhantomData,
    }
  }

  pub fn push_front(&mut self, elem: T) {
    unsafe {
      let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
        front: None,
        back: None,
        elem,
      })));

      match self.front {
        // List is not empty and it has a head.
        Some(old) => {
          (*old.as_ptr()).front = Some(new);
          (*new.as_ptr()).back = Some(old);
        }
        // List is empty so it doesn't have a head.
        None => {
          self.back = Some(new);
        }
      }

      self.front = Some(new);
      self.len += 1;
    }
  }

  pub fn push_back(&mut self, elem: T) {
    // SAFETY: it's a linked-list, what do you want?
    unsafe {
      let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
        back: None,
        front: None,
        elem,
      })));
      if let Some(old) = self.back {
        // Put the new back before the old one
        (*old.as_ptr()).back = Some(new);
        (*new.as_ptr()).front = Some(old);
      } else {
        // If there's no back, then we're the empty list and need
        // to set the front too.
        self.front = Some(new);
      }
      // These things always happen!
      self.back = Some(new);
      self.len += 1;
    }
  }

  pub fn pop_front(&mut self) -> Option<T> {
    unsafe {
      self.front.map(|node| {
        // Create a Box from the pointer so that it is deallocated automatically
        // when the Box is dropped.
        let boxed_node = Box::from_raw(node.as_ptr());

        let result = boxed_node.elem;

        self.front = boxed_node.back;

        match self.front {
          Some(new) => {
            (*new.as_ptr()).front = None;
          }
          None => {
            self.back = None;
          }
        }

        self.len -= 1;

        result
      })
    }
  }

  pub fn pop_back(&mut self) -> Option<T> {
    unsafe {
      // Only have to do stuff if there is a back node to pop.
      self.back.map(|node| {
        // Bring the Box front to life so we can move out its value and
        // Drop it (Box continues to magically understand this for us).
        let boxed_node = Box::from_raw(node.as_ptr());
        let result = boxed_node.elem;

        // Make the next node into the new back.
        self.back = boxed_node.front;
        if let Some(new) = self.back {
          // Cleanup its reference to the removed node
          (*new.as_ptr()).back = None;
        } else {
          // If the back is now null, then this list is now empty!
          self.front = None;
        }

        self.len -= 1;
        result
        // Box gets implicitly freed here, knows there is no T.
      })
    }
  }

  pub fn front(&self) -> Option<&T> {
    unsafe { self.front.map(|node| &(*node.as_ptr()).elem) }
  }

  pub fn front_mut(&mut self) -> Option<&mut T> {
    unsafe { self.front.map(|node| &mut (*node.as_ptr()).elem) }
  }

  pub fn back(&self) -> Option<&T> {
    unsafe { self.back.map(|node| &(*node.as_ptr()).elem) }
  }

  pub fn back_mut(&mut self) -> Option<&mut T> {
    unsafe { self.back.map(|node| &mut (*node.as_ptr()).elem) }
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  pub fn clear(&mut self) {
    // Pop nodes until the list becomes empty.
    while self.pop_front().is_some() {}
  }

  pub fn iter(&self) -> Iter<T> {
    Iter {
      front: self.front,
      back: self.back,
      len: self.len,
      _p: PhantomData,
    }
  }

  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter { list: self }
  }

  pub fn iter_mut(&mut self) -> IterMut<T> {
    IterMut {
      front: self.front,
      back: self.back,
      len: self.len,
      _p: PhantomData,
    }
  }

  pub fn cursor_mut(&mut self) -> CursorMut<T> {
    CursorMut {
      current: None,
      list: self,
      index: None,
    }
  }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Send> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Send> Sync for IterMut<'a, T> {}

impl<T> Drop for LinkedList<T> {
  fn drop(&mut self) {
    // Pop nodes until the list is empty.
    while self.pop_front().is_some() {}
  }
}

impl<T> Default for LinkedList<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Clone> Clone for LinkedList<T> {
  fn clone(&self) -> Self {
    let mut new_list = Self::new();

    for item in self.iter() {
      new_list.push_back(item.clone());
    }

    new_list
  }
}

impl<T> Extend<T> for LinkedList<T> {
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = T>,
  {
    for item in iter.into_iter() {
      self.push_back(item);
    }
  }
}

impl<T> FromIterator<T> for LinkedList<T> {
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = T>,
  {
    let mut list = Self::new();
    list.extend(iter);
    list
  }
}

impl<T: Debug> Debug for LinkedList<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_list().entries(self).finish()
  }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
  fn eq(&self, other: &Self) -> bool {
    self.len() == other.len() && self.iter().eq(other)
  }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.iter().partial_cmp(other)
  }
}

impl<T: Ord> Ord for LinkedList<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.iter().cmp(other)
  }
}

impl<T: Hash> Hash for LinkedList<T> {
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.len().hash(state);

    for item in self.iter() {
      item.hash(state);
    }
  }
}

pub struct Iter<'a, T> {
  front: Link<T>,
  back: Link<T>,
  len: usize,
  _p: PhantomData<&'a T>,
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
  type IntoIter = Iter<'a, T>;
  type Item = &'a T;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    // TODO: is this needed since we are calling Option::map?
    if self.len == 0 {
      return None;
    }

    self.front.map(|node| unsafe {
      self.len -= 1;
      self.front = (*node.as_ptr()).back;
      &(*node.as_ptr()).elem
    })
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len, Some(self.len))
  }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    // TODO: is this needed since we are calling Option::map?
    if self.len == 0 {
      return None;
    }

    self.back.map(|node| unsafe {
      self.len -= 1;
      self.back = (*node.as_ptr()).front;
      &(*node.as_ptr()).elem
    })
  }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
  fn len(&self) -> usize {
    self.len
  }
}

pub struct IntoIter<T> {
  list: LinkedList<T>,
}

impl<T> IntoIterator for LinkedList<T> {
  type IntoIter = IntoIter<T>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    self.into_iter()
  }
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.list.pop_front()
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.list.len, Some(self.list.len))
  }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.list.pop_back()
  }
}

impl<T> ExactSizeIterator for IntoIter<T> {
  fn len(&self) -> usize {
    self.list.len
  }
}

pub struct IterMut<'a, T> {
  front: Link<T>,
  back: Link<T>,
  len: usize,
  _p: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    // While self.front == self.back is a tempting condition to check here,
    // it won't do the right for yielding the last element! That sort of
    // thing only works for arrays because of "one-past-the-end" pointers.
    if self.len > 0 {
      // We could unwrap front, but this is safer and easier
      self.front.map(|node| unsafe {
        self.len -= 1;
        self.front = (*node.as_ptr()).back;
        &mut (*node.as_ptr()).elem
      })
    } else {
      None
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.len, Some(self.len))
  }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    if self.len > 0 {
      self.back.map(|node| unsafe {
        self.len -= 1;
        self.back = (*node.as_ptr()).front;
        &mut (*node.as_ptr()).elem
      })
    } else {
      None
    }
  }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
  fn len(&self) -> usize {
    self.len
  }
}

pub struct CursorMut<'a, T> {
  current: Link<T>,
  list: &'a mut LinkedList<T>,
  index: Option<usize>,
}

impl<'a, T> CursorMut<'a, T> {
  pub fn index(&self) -> Option<usize> {
    self.index
  }

  pub fn move_next(&mut self) {
    if let Some(current) = self.current {
      unsafe {
        self.current = (*current.as_ptr()).back;

        if self.current.is_some() {
          *self.index.as_mut().unwrap() += 1;
        } else {
          self.index = None;
        }
      }
    } else if !self.list.is_empty() {
      self.current = self.list.front;
      self.index = Some(0);
    }
  }

  pub fn move_prev(&mut self) {
    if let Some(current) = self.current {
      unsafe {
        self.current = (*current.as_ptr()).front;

        if self.current.is_some() {
          *self.index.as_mut().unwrap() += 1;
        } else {
          self.index = None;
        }
      }
    } else if !self.list.is_empty() {
      self.current = self.list.back;
      self.index = Some(self.list.len - 1);
    }
  }

  pub fn current(&mut self) -> Option<&mut T> {
    unsafe { self.current.map(|node| &mut (*node.as_ptr()).elem) }
  }

  pub fn peek_next(&mut self) -> Option<&mut T> {
    unsafe {
      self
        .current
        .and_then(|node| (*node.as_ptr()).back)
        .map(|node| &mut (*node.as_ptr()).elem)
    }
  }

  pub fn peek_prev(&mut self) -> Option<&mut T> {
    unsafe {
      self
        .current
        .and_then(|node| (*node.as_ptr()).front)
        .map(|node| &mut (*node.as_ptr()).elem)
    }
  }

  pub fn split_before(&mut self) -> LinkedList<T> {
    match self.current {
      // We are pointing to a real element, so the list is non-empty.
      Some(current) => {
        unsafe {
          // Current state
          let old_len = self.list.len;
          let old_index = self.index.unwrap();
          let previous = (*current.as_ptr()).front;

          // What self will become
          let new_len = old_len - old_index;
          let new_front = self.current;
          let new_back = self.list.back;
          let new_index = Some(0);

          // What the output will become
          let output_len = old_len - new_len;
          let output_front = self.list.front;
          let output_back = previous;

          // Break the links between current and previous
          if let Some(previous) = previous {
            (*current.as_ptr()).front = None;
            (*previous.as_ptr()).back = None;
          }

          // Product the result
          self.list.len = new_len;
          self.list.front = new_front;
          self.list.back = new_back;
          self.index = new_index;

          LinkedList {
            front: output_front,
            back: output_back,
            len: output_len,
            _p: PhantomData,
          }
        }
      }
      None => {
        // We're at the gost, just replace our list with an empty one.
        std::mem::replace(self.list, LinkedList::new())
      }
    }
  }

  pub fn splice_before(&mut self, mut input: LinkedList<T>) {
    if input.is_empty() {
      return;
    }

    unsafe {
      if let Some(current) = self.current {
        if let Some(0) = self.index {
          // We're appending to the front
          (*current.as_ptr()).front = input.back.take();
          (*input.back.unwrap().as_ptr()).back = Some(current);
          self.list.front = input.front.take();
        } else {
          let previous = (*current.as_ptr()).front.unwrap();
          let in_front = input.front.take().unwrap();
          let in_back = input.back.take().unwrap();

          (*previous.as_ptr()).back = Some(in_front);
          (*in_front.as_ptr()).front = Some(previous);
          (*current.as_ptr()).front = Some(in_back);
          (*in_back.as_ptr()).back = Some(current);
        }

        *self.index.as_mut().unwrap() += input.len;
        self.list.len += input.len;
        input.len = 0;
      } else if let Some(back) = self.list.back {
        (*back.as_ptr()).back = input.front.take();
        (*input.front.unwrap().as_ptr()).front = Some(back);
        self.list.back = input.back.take();
        self.list.len += input.len;
        input.len = 0;
      } else {
        std::mem::swap(self.list, &mut input);
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn generate_test() -> LinkedList<i32> {
    list_from(&[0, 1, 2, 3, 4, 5, 6])
  }

  fn list_from<T: Clone>(v: &[T]) -> LinkedList<T> {
    v.iter().map(|x| (*x).clone()).collect()
  }

  #[test]
  fn test_basic_front() {
    let mut list = LinkedList::new();

    // Try to break an empty list
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);

    // Try to break a one item list
    list.push_front(10);
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop_front(), Some(10));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);

    // Mess around
    list.push_front(10);
    assert_eq!(list.len(), 1);
    list.push_front(20);
    assert_eq!(list.len(), 2);
    list.push_front(30);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(30));
    assert_eq!(list.len(), 2);
    list.push_front(40);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(40));
    assert_eq!(list.len(), 2);
    assert_eq!(list.pop_front(), Some(20));
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop_front(), Some(10));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);
  }

  #[test]
  fn test_basic() {
    let mut m = LinkedList::new();
    assert_eq!(m.pop_front(), None);
    assert_eq!(m.pop_back(), None);
    assert_eq!(m.pop_front(), None);
    m.push_front(1);
    assert_eq!(m.pop_front(), Some(1));
    m.push_back(2);
    m.push_back(3);
    assert_eq!(m.len(), 2);
    assert_eq!(m.pop_front(), Some(2));
    assert_eq!(m.pop_front(), Some(3));
    assert_eq!(m.len(), 0);
    assert_eq!(m.pop_front(), None);
    m.push_back(1);
    m.push_back(3);
    m.push_back(5);
    m.push_back(7);
    assert_eq!(m.pop_front(), Some(1));

    let mut n = LinkedList::new();
    n.push_front(2);
    n.push_front(3);
    {
      assert_eq!(n.front().unwrap(), &3);
      let x = n.front_mut().unwrap();
      assert_eq!(*x, 3);
      *x = 0;
    }
    {
      assert_eq!(n.back().unwrap(), &2);
      let y = n.back_mut().unwrap();
      assert_eq!(*y, 2);
      *y = 1;
    }
    assert_eq!(n.pop_front(), Some(0));
    assert_eq!(n.pop_front(), Some(1));
  }

  #[test]
  fn test_iterator() {
    let m = generate_test();
    for (i, elt) in m.iter().enumerate() {
      assert_eq!(i as i32, *elt);
    }
    let mut n = LinkedList::new();
    assert_eq!(n.iter().next(), None);
    n.push_front(4);
    let mut it = n.iter();
    assert_eq!(it.size_hint(), (1, Some(1)));
    assert_eq!(it.next().unwrap(), &4);
    assert_eq!(it.size_hint(), (0, Some(0)));
    assert_eq!(it.next(), None);
  }

  #[test]
  fn test_iterator_double_end() {
    let mut n = LinkedList::new();
    assert_eq!(n.iter().next(), None);
    n.push_front(4);
    n.push_front(5);
    n.push_front(6);
    let mut it = n.iter();
    assert_eq!(it.size_hint(), (3, Some(3)));
    assert_eq!(it.next().unwrap(), &6);
    assert_eq!(it.size_hint(), (2, Some(2)));
    assert_eq!(it.next_back().unwrap(), &4);
    assert_eq!(it.size_hint(), (1, Some(1)));
    assert_eq!(it.next_back().unwrap(), &5);
    assert_eq!(it.next_back(), None);
    assert_eq!(it.next(), None);
  }

  #[test]
  fn test_rev_iter() {
    let m = generate_test();
    for (i, elt) in m.iter().rev().enumerate() {
      assert_eq!(6 - i as i32, *elt);
    }
    let mut n = LinkedList::new();
    assert_eq!(n.iter().rev().next(), None);
    n.push_front(4);
    let mut it = n.iter().rev();
    assert_eq!(it.size_hint(), (1, Some(1)));
    assert_eq!(it.next().unwrap(), &4);
    assert_eq!(it.size_hint(), (0, Some(0)));
    assert_eq!(it.next(), None);
  }

  #[test]
  fn test_mut_iter() {
    let mut m = generate_test();
    let mut len = m.len();
    for (i, elt) in m.iter_mut().enumerate() {
      assert_eq!(i as i32, *elt);
      len -= 1;
    }
    assert_eq!(len, 0);
    let mut n = LinkedList::new();
    assert!(n.iter_mut().next().is_none());
    n.push_front(4);
    n.push_back(5);
    let mut it = n.iter_mut();
    assert_eq!(it.size_hint(), (2, Some(2)));
    assert!(it.next().is_some());
    assert!(it.next().is_some());
    assert_eq!(it.size_hint(), (0, Some(0)));
    assert!(it.next().is_none());
  }

  #[test]
  fn test_iterator_mut_double_end() {
    let mut n = LinkedList::new();
    assert!(n.iter_mut().next_back().is_none());
    n.push_front(4);
    n.push_front(5);
    n.push_front(6);
    let mut it = n.iter_mut();
    assert_eq!(it.size_hint(), (3, Some(3)));
    assert_eq!(*it.next().unwrap(), 6);
    assert_eq!(it.size_hint(), (2, Some(2)));
    assert_eq!(*it.next_back().unwrap(), 4);
    assert_eq!(it.size_hint(), (1, Some(1)));
    assert_eq!(*it.next_back().unwrap(), 5);
    assert!(it.next_back().is_none());
    assert!(it.next().is_none());
  }

  #[test]
  fn test_eq() {
    let mut n: LinkedList<u8> = list_from(&[]);
    let mut m = list_from(&[]);
    assert!(n == m);
    n.push_front(1);
    assert!(n != m);
    m.push_back(1);
    assert!(n == m);

    let n = list_from(&[2, 3, 4]);
    let m = list_from(&[1, 2, 3]);
    assert!(n != m);
  }

  #[allow(clippy::eq_op)]
  #[test]
  fn test_ord() {
    let n = list_from(&[]);
    let m = list_from(&[1, 2, 3]);
    assert!(n < m);
    assert!(m > n);
    assert!(n <= n);
    assert!(n >= n);
  }

  #[allow(clippy::eq_op)]
  #[test]
  fn test_ord_nan() {
    let nan = 0.0f64 / 0.0;
    let n = list_from(&[nan]);
    let m = list_from(&[nan]);
    assert!(!(n < m));
    assert!(!(n > m));
    assert!(!(n <= m));
    assert!(!(n >= m));

    let n = list_from(&[nan]);
    let one = list_from(&[1.0f64]);
    assert!(!(n < one));
    assert!(!(n > one));
    assert!(!(n <= one));
    assert!(!(n >= one));

    let u = list_from(&[1.0f64, 2.0, nan]);
    let v = list_from(&[1.0f64, 2.0, 3.0]);
    assert!(!(u < v));
    assert!(!(u > v));
    assert!(!(u <= v));
    assert!(!(u >= v));

    let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);
    let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);
    assert!(!(s < t));
    assert!(s > one);
    assert!(!(s <= one));
    assert!(s >= one);
  }

  #[test]
  fn test_debug() {
    let list: LinkedList<i32> = (0..10).collect();
    assert_eq!(format!("{:?}", list), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

    let list: LinkedList<&str> = vec!["just", "one", "test", "more"]
      .iter()
      .copied()
      .collect();
    assert_eq!(format!("{:?}", list), r#"["just", "one", "test", "more"]"#);
  }

  #[test]
  fn test_hashmap() {
    // Check that HashMap works with this as a key

    let list1: LinkedList<i32> = (0..10).collect();
    let list2: LinkedList<i32> = (1..11).collect();
    let mut map = std::collections::HashMap::new();

    assert_eq!(map.insert(list1.clone(), "list1"), None);
    assert_eq!(map.insert(list2.clone(), "list2"), None);

    assert_eq!(map.len(), 2);

    assert_eq!(map.get(&list1), Some(&"list1"));
    assert_eq!(map.get(&list2), Some(&"list2"));

    assert_eq!(map.remove(&list1), Some("list1"));
    assert_eq!(map.remove(&list2), Some("list2"));

    assert!(map.is_empty());
  }

  #[allow(dead_code)]
  fn assert_properties() {
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    is_send::<LinkedList<i32>>();
    is_sync::<LinkedList<i32>>();

    is_send::<IntoIter<i32>>();
    is_sync::<IntoIter<i32>>();

    is_send::<Iter<i32>>();
    is_sync::<Iter<i32>>();

    is_send::<IterMut<i32>>();
    is_sync::<IterMut<i32>>();

    // is_send::<Cursor<i32>>();
    // is_sync::<Cursor<i32>>();

    fn linked_list_covariant<'a, T>(x: LinkedList<&'static T>) -> LinkedList<&'a T> {
      x
    }
    fn iter_covariant<'i, 'a, T>(x: Iter<'i, &'static T>) -> Iter<'i, &'a T> {
      x
    }
    fn into_iter_covariant<'a, T>(x: IntoIter<&'static T>) -> IntoIter<&'a T> {
      x
    }
  }

  /// ```compile_fail
  /// use super::*;
  ///
  /// fn iter_mut_covariant<'i, 'a, T>(x: IterMut<'i, &'static T>) -> IterMut<'i, &'a T> {
  ///   x
  /// }
  /// ```
  fn iter_mut_covariant() {}
}