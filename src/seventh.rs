pub struct List<'a, T> {
  pub data: T,
  pub prev: Option<&'a List<'a, T>>,
}

impl<'a, T> List<'a, T> {
  pub fn push<U>(prev: Option<&'a List<'a, T>>, data: T, callback: impl FnOnce(&List<'a, T>) -> U) {
    let list = List { data, prev };
    callback(&list);
  }

  pub fn iter(&'a self) -> Iter<'a, T> {
    Iter { next: Some(self) }
  }
}

pub struct Iter<'a, T> {
  next: Option<&'a List<'a, T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    let node = self.next?;
    self.next = node.prev;
    Some(&node.data)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn elegance() {
    List::push(None, 3, |list| {
      assert_eq!(list.iter().copied().sum::<i32>(), 3);

      List::push(Some(list), 5, |list| {
        assert_eq!(list.iter().copied().sum::<i32>(), 5 + 3);

        List::push(Some(list), 13, |list| {
          assert_eq!(list.iter().copied().sum::<i32>(), 13 + 5 + 3);
        })
      })
    })
  }
}
