use crate::{HNext, IntoHCons};

pub trait Vars<T>: PushFront<T> + HNext<T> + IntoHCons<T, Self::Cons> {
    type Cons: HNext<T>;
}

pub trait Tuple: UnCons {}

pub trait UnCons {
    type Head;
    type Tail;

    fn uncons(self) -> (Self::Head, Self::Tail);
}

pub trait UnConsOpt<T> {
    type Head;
    type Tail;

    fn uncons_opt(self) -> (Self::Head, Self::Tail);
}

pub trait UnConsMut {
    type Head<'h>
    where
        Self: 'h;
    type Tail<'t>
    where
        Self: 't;

    fn uncons_mut<'a>(&'a mut self) -> (Self::Head<'a>, Self::Tail<'a>);
}

pub trait PopFront<E>: Vars<E> {
    fn pop_front(&mut self) -> E;
}

// impl<Src: Vars<usize> + Copy> PopFront<usize> for Src {
//     fn pop_front(&mut self) -> usize {
//         let (h, t) = self.uncons();
//         h
//     }
// }

pub trait GetFront {
    type Output<'a>
    where
        Self: 'a;

    fn get_front<'b>(&'b self) -> Self::Output<'b>;
}

pub trait GetFrontMut {
    type Output<'a>
    where
        Self: 'a;

    fn get_front<'b>(&mut self) -> Self::Output<'b>;
}

pub trait PushFront<E> {
    type Output;

    fn push_front(self, e: E) -> Self::Output;
}

pub trait PushBack<E> {
    type Output;

    fn push_back(self, e: E) -> Self::Output;
}

pub trait Compose<Init> {
    type Output;

    fn compose(self, init: Init) -> Self::Output;
}

pub trait ComposeMut<Init> {
    type Output;

    fn compose_mut(self, init: Init) -> Self::Output;
}

pub trait DerefTuple {
    type Output;

    fn deref(self) -> Self::Output;
}

pub trait DerefTupleMut {
    type Output;

    fn deref_mut(&mut self) -> Self::Output;
}

impl<T1: Copy, T2: Copy> crate::DerefTuple for (&T1, &T2) {
    type Output = (T1, T2);

    fn deref(self) -> Self::Output {
        // let (t1, t2) = *self;
        (self.0.to_owned(), self.1.to_owned())
    }
}
