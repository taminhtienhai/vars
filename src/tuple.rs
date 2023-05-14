use core::*;

macro_rules! impl_trait_for_tuple {
    () => {};
    ($head:ident $($tails:ident)*) => {
        impl_trait_for_tuple!($($tails)*);

        impl <$head, $($tails,)*> crate::Tuple for ($head, $($tails,)*) {}
        impl <E, $head, $($tails,)*> crate::TupleG<E> for ($head, $($tails,)*) {}

        impl <$head, $($tails,)*> crate::UnCons for ($head, $($tails,)*) {
            type Head = $head;
            type Tail = ($($tails,) *);
            type Output = (Self::Head, Self::Tail);

            #[allow(non_snake_case)]
            fn uncons(self) -> Self::Output {
                let (h, $($tails,) *) = self;
                (h, ($($tails,) *))
            }
        }

        impl <H, $head, $($tails,)*> crate::PushFront<H> for ($head, $($tails,)*) {
            type Output = (H, $head, $($tails,)*);

            fn push_front(self, e: H) -> Self::Output {
                let (h, $($tails,)*) = self;
                (e, h, $($tails,)*)
            }
        }

        // impl <Init, $head, $($tails,)*> crate::Compose<Init> for ($head, $($tails,)*) {
        //     type Head = Fn($head, )
        // }
    };
}

impl_trait_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);