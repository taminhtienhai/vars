use crate::{HNext, UnCons};

macro_rules! impl_trait_for_tuple {
    () => {};
    ($head:ident $($tails:ident)*) => {
        impl_trait_for_tuple!($($tails )*);

        impl <$head, $($tails,)*> crate::Tuple for ($head, $($tails,)*) {}

        impl <$head, $($tails,)*> crate::UnCons for ($head, $($tails,)*) {
            type Head = $head;
            type Tail = ($($tails,) *);

            #[allow(non_snake_case)]
            fn uncons(self) -> (Self::Head, Self::Tail) {
                let (h, $($tails,) *) = self;
                (h, ($($tails,) *))
            }
        }

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
    };
}

impl_trait_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);


impl<T> HNext<T> for () {
    type Next = ();

    fn value(&self) -> Option<&T> {
        None
    }

    fn next(self) -> Self::Next {
        ()
    }
}