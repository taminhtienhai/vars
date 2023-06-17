extern crate alloc;

use core::mem::MaybeUninit;

use alloc::rc::Rc;

use crate::{HNext, UnCons};

pub struct TupleIter<T> {
    data: Rc<[MaybeUninit<T>]>,
    cur: usize,
}

impl <T> Iterator for crate::TupleIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_pos = self.cur;
        if self.cur >= self.data.len() { return None; }
        self.cur += 1;
        Some(unsafe { self.data.get_unchecked(current_pos).assume_init_read() })
    }
}

macro_rules! impl_trait_for_tuple {
    () => {};
    ($head:ident $($tails:ident)*) => {
        impl_trait_for_tuple!($($tails )*);

        // impl <$head, $($tails,)*> crate::Tuple for ($head, $($tails,)*) {}

        crate::impl_uncons_for_tuple!($head, $($tails,)*);

        // impl <$head, $($tails,)*> crate::DerefTuple for (&$head, $(&$tails,)*) {
        //     type Output = ($head, $($tails,)*);

        //     fn deref(&self) -> Self::Output {
        //         let (h, $($tails,)*) = self;
        //         (*h, $(*$tails,)*)
        //     }
        // }

        impl <$head, $($tails,)*> crate::UnConsMut for ($head, $($tails,)*) {
            type Head<'h> = &'h mut $head where $head: 'h, $($tails: 'h,)*;
            type Tail<'t> = ($(&'t mut $tails,) *) where $head: 't, $($tails: 't,)*;

            #[allow(non_snake_case)]
            fn uncons_mut<'a>(&'a mut self) -> (Self::Head<'a>, Self::Tail<'a>) {
                let (h, $($tails,) *) = self;
                (h, ($($tails,) *))
            }
        }

        // impl <$head, $($tails,)*> crate::PopFront<$head> for ($head, $($tails,)*) {
        //     fn pop_front(self) -> $head {
        //         self.0
        //     }
        // }

        impl <H, $head, $($tails,)*> crate::PushFront<H> for ($head, $($tails,)*) {
            type Output = (H, $head, $($tails,)*);

            #[allow(non_snake_case)]
            fn push_front(self, e: H) -> Self::Output {
                let (h, $($tails,)*) = self;
                (e, h, $($tails,)*)
            }
        }

        crate::impl_compose_fn!($head, $($tails,)*);
        crate::impl_vars_for_tuple!($head, $($tails,)*);


        crate::impl_hnext_for_tuple!($head, $($tails,)*);
        crate::impl_into_hcons_for_tuple!($head, $($tails,)*);
        crate::impl_iterator_for_tuple!($head, $($tails,)*);
    };
}

impl_trait_for_tuple!(T1 T2 T3 T4);

impl <T> UnCons<T> for () {
    type NextIt = T;
    type Tail = ();

    fn uncons(self) -> (Option<T>, Self::Tail) {
        (None, ())
    }
}


impl<T> HNext<T> for () {
    type Next = ();

    fn value(&self) -> Option<&T> {
        None
    }

    fn next(self) -> Self::Next {
        ()
    }
}