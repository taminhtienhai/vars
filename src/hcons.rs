#[derive(Clone, PartialEq, Eq)]
pub struct HCons<L, R: HNext<L>>(pub L, pub R);

pub struct HEmpty<T> {
    pub _phantom: core::marker::PhantomData<T>,
}

pub trait HNext<T> {
    type Next: HNext<T>;

    fn value(&self) -> Option<&T>;

    fn next(self) -> Self::Next;
}

pub trait IntoHCons<L, R: HNext<L>> {
    fn into_hcons(self) -> HCons<L, R>;
}

impl<T> HNext<T> for HEmpty<T> {
    type Next = Self;

    fn value(&self) -> Option<&T> {
        None
    }

    fn next(self) -> Self::Next {
        self
    }
}

impl<T, E: HNext<T>> HNext<T> for HCons<T, E> {
    type Next = E;

    fn value(&self) -> Option<&T> {
        Some(&self.0)
    }

    fn next(self) -> Self::Next {
        self.1
    }
}

// pub trait IntoHCons {
//     type Output;

//     fn into_hcons(self) -> Self::Output; 
// }


// impl <T1,T2> IntoHCons for (T1, T2,) {
//     type Output = HCons<T1, HCons<T2, ()>>;

//     fn into_hcons(self) -> Self::Output {
//         HCons(self.0, HCons(self.1, ()))
//     }
// }

// impl HEmpty for () {}

// impl <T: HEmpty> crate::UnCons for T {
//     type Head = Option<usize>;
//     type Tail = ();

//     fn uncons(self) -> (Self::Head, Self::Tail) {
//         (None, ())
//     }

// }

// impl <L,R: crate::UnCons> crate::UnCons for HCons<L,R> {
//     type Head = Option<L>;
//     type Tail = R;

//     fn uncons(self) -> (Self::Head, Self::Tail) {
//         (Some(self.0), self.1)
//     }
// } 

// impl <E> crate::PushFront<E> for HEmpty {
//     type Output = HCons<E, HEmpty>;

//     fn push_front(self, e: E) -> Self::Output {
//         HCons(e, HEmpty)
//     }
// }

// impl <G,V: Vars<G>> crate::UnConsOpt<G> for V {
//     type Head = Option<G>;

//     type Tail = V::Tail;

//     fn uncons_opt(self) -> (Self::Head, Self::Tail) {
//         let (a,b) = self.uncons();

//         (Some(a), b)
//     }
// }