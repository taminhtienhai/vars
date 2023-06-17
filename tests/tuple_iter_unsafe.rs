// trait Next<T: Clone> {
//     type Tail: Next<T>;
//     fn next(&mut self) -> (Option<T>, Self::Tail);
// }

// impl <T: Clone> Next<T> for () {
//     type Tail = ();

//     fn next(&mut self) -> (Option<T>, Self::Tail) {
//         (None, ())
//     }
// }

// impl <T: Clone> Next<T> for (T,) {
//     type Tail = ();

//     fn next(&mut self) -> (Option<T>, Self::Tail) {
//         (Some(self.0.clone()), ())
//     }
// }

use std::{rc::Rc, mem::{MaybeUninit, transmute_copy}};

struct TupleIter<T> {
    data: Rc<[MaybeUninit<T>]>,
    cur: usize,
}

impl <T> Iterator for TupleIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_pos = self.cur;
        if self.cur >= self.data.len() { return None; }
        self.cur += 1;
        Some(unsafe { self.data.get_unchecked(current_pos).assume_init_read() })
    }
}

trait TupleIntoIterator<T> {
    type Output;
    fn into_iter(self) -> Self::Output;
}

trait TupleIterator<T> {
    type Output;
    fn iter(&self) -> Self::Output;
}

impl <T> TupleIntoIterator<T> for (T,T,) {
    type Output = TupleIter<T>;

    fn into_iter(self) -> Self::Output {
        TupleIter {
            data: Rc::<[MaybeUninit<T>; 2]>::new(unsafe {
                transmute_copy(&self)
            }),
            cur: 0,
        }
    }
}

// impl <T> TupleIterator<T> for (T,T,) {
//     type Output = TupleIter<T>;

//     fn iter(&self) -> Self::Output {
//         TupleIter {
//             data: Rc::new([self.0, self.1]),
//             cur: 0,
//         }
//     }
// }


#[test]
fn tuple_iter() {
    let mut tups = (1 as usize, 2 as usize,);

    // tups.0 = 3 as usize;

    for i in tups.into_iter() {
        println!("{:?}", i);
    }

    // tups.0 = 4 as usize;

    println!("{tups:?}");
}