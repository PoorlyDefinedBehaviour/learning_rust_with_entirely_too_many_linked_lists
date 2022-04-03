//! Run tests with:
//!
//! ```terminal
//! MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo +nightly-2022-01-21 miri test
//! ```
//!
//! [a, b, c] means a is the element at the top of the stacked borrows.

#[cfg(test)]
mod tests {
  #[test]
  fn not_a_miri_example() {
    /*
    let mut data = 10;
    let ref1 = &mut data; // StackedBorrows = [ref1]
    let ref2 = &mut *ref1; // StackedBorrows = [ref2, ref1]

    // ORDER SWAPPED!
    *ref1 += 1; // StackedBorrows = [ref2, ref1] (error because ref1 is not at the stop of stack)
    *ref2 += 2;

    println!("{}", data);
    //     error[E0503]: cannot use `*ref1` because it was mutably borrowed
    //   --> src/miri.rs:20:5
    //    |
    // 17 |     let ref2 = &mut *ref1; // StackedBorrows = [ref2, ref1]
    //    |                ---------- borrow of `*ref1` occurs here
    // ...
    // 20 |     *ref1 += 1; // StackedBorrows = [ref2, ref1] (error because ref1 is not at the stop of stack)
    //    |     ^^^^^^^^^^ use of borrowed `*ref1`
    // 21 |     *ref2 += 2;
    //    |     ---------- borrow later used here
    */
  }

  #[test]
  fn miri_example_1() {
    unsafe {
      let mut data = 10;
      let ref1 = &mut data; // StackedBorrows = [ref1]
      let ptr2 = ref1 as *mut _; // StackedBorrows = [ptr2, ref1]

      // Swaping the order of these two assignments would make the code work.
      *ref1 += 1; // StackedBorrows = [ref1] (pops ptr2 to get to ref1 to the top)
      *ptr2 += 2; // StackedBorrows = [ref1] (ptr2 is not a the top of the stack, so that's an error)

      println!("{}", data);
    }
    //     test miri::tests::miri_example_1 ... error: Undefined Behavior: no item granting read access to tag <203447> at alloc76582 found in borrow stack.
    //   --> src/miri.rs:19:7
    //    |
    // 19 |       *ptr2 += 2;
    //    |       ^^^^^^^^^^ no item granting read access to tag <203447> at alloc76582 found in borrow stack.
    //    |
    //    = help: this indicates a potential bug in the program: it performed an invalid operation, but the rules it violated are still experimental
    //    = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
    //    = note: inside `miri::tests::miri_example_1` at src/miri.rs:19:7
  }

  #[test]
  fn miri_example_2() {
    unsafe {
      let mut data = 10;
      let ref1 = &mut data; // StackedBorrows = [ref1]
      let ptr2 = ref1 as *mut _; // StackedBorrows = [ptr2, ref1]
      let ref3 = &mut *ptr2; // StackedBorrows = [ref3, ptr2, ref1]
      let ptr4 = ref3 as *mut _; // StackedBorrows = [ptr4, ref3, ptr2, ref1]

      // Access the first raw pointer first
      *ptr2 += 2; // StackedBorrows = [ptr2, ref1] (ptr4 and ref3 have been popped)

      // Then access things in "borrow stack" order
      *ptr4 += 4; // StackedBorrows = [ptr2, ref1] (error because ptr4 is not in the stack)
      *ref3 += 3;
      *ptr2 += 2;
      *ref1 += 1;

      println!("{}", data);
    }
    //     test miri::tests::miri_example_2 ... error: Undefined Behavior: no item granting read access to tag <206975> at alloc77862 found in borrow stack.
    //   --> src/miri.rs:50:7
    //    |
    // 50 |       *ptr4 += 4; // StackedBorrows = [ptr2, ref1] (error because ptr4 not in the stack)
    //    |       ^^^^^^^^^^ no item granting read access to tag <206975> at alloc77862 found in borrow stack.
    //    |
    //    = help: this indicates a potential bug in the program: it performed an invalid operation, but the rules it violated are still experimental
    //    = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
    //    = note: inside `miri::tests::miri_example_2` at src/miri.rs:50:7
  }
}
