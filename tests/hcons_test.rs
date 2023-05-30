// use std::{marker::PhantomData, ops::Deref};

use std::marker::PhantomData;

use vars::UnCons;

#[derive(Default)]
struct HEmpty<T> {
    _phantom: std::marker::PhantomData<T>,
}
struct HCons<L, R: HNext<L>>(L, R);

trait HNext<T> {
    type Next: HNext<T>;

    fn value(&self) -> Option<&T>;

    fn next(self) -> Self::Next;
}

trait IntoHCons<L, R: HNext<L>> {
    fn into_hcons(self) -> HCons<L, R>;
}

trait Vars<T>: HNext<T> + IntoHCons<T, Self::Cons> {
    type Cons: HNext<T>;
    // type Output: HNext<T>;
    // fn into_hcons(self) -> Self::Output;
}

// : IntoHCons<T, HCons<T, HEmpty<T>>> + IntoHCons<T, HCons<T,HCons<T, HEmpty<T>>>>{

// impl <T> IntoHCons<T, HEmpty<T>> for () {
//     fn into_hcons(self) -> HEmpty<T> {
//         HEmpty { _phantom: std::marker::PhantomData::default() }
//     }
// }

impl<T> IntoHCons<T, HEmpty<T>> for (T,) {
    fn into_hcons(self) -> HCons<T, HEmpty<T>> {
        HCons(
            self.0,
            HEmpty::<T> { _phantom: PhantomData::<T>::default() },
        )
    }
}

impl<T> IntoHCons<T, HCons<T, HEmpty<T>>> for (T, T) {
    fn into_hcons(self) -> HCons<T, HCons<T, HEmpty<T>>> {
        HCons(
            self.0,
            HCons(
                self.1,
                HEmpty::<T> { _phantom: PhantomData::<T>::default() },
            ),
        )
    }
}

// impl <T,G: Vars<T>> IntoHCons<T, HCons<T, HEmpty<T>>> for G {
//     fn into_hcons(self) -> HCons<T,HCons<T, HEmpty<T>>> {
//         HCons(self.0, HCons(self.1, HEmpty { _phantom: std::marker::PhantomData::default() }))
//     }
// }

// impl <T> IntoHCons<T, HCons<T,HCons<T, HEmpty<T>>>> for (T,T,T,) {
//     fn into_hcons(self) -> HCons<T,HCons<T,HCons<T, HEmpty<T>>>> {
//         HCons(self.0, HCons(self.1, HCons(self.2, HEmpty { _phantom: std::marker::PhantomData::default() })))
//     }
// }

// impl <G, T: Vars<G>> IntoHCons<G,HCons<G, HEmpty<G>>> for T {
//     fn into_hcons(self) -> HCons<G,T> {
//         self.into_hcons()
//     }
// }

impl<T> HNext<T> for () {
    type Next = ();

    fn value(&self) -> Option<&T> {
        None
    }

    fn next(self) -> Self::Next {
        ()
    }
}

impl<T> HNext<T> for (T,) {
    type Next = ();

    fn value(&self) -> Option<&T> {
        Some(&self.0)
    }

    fn next(self) -> Self::Next {
        ()
    }
}

impl<T> HNext<T> for (T, T) {
    type Next = (T,);

    fn value(&self) -> Option<&T> {
        Some(&self.0)
    }

    fn next(self) -> (T,) {
        (self.1,)
    }
}

impl<T> Vars<T> for (T,) {
    type Cons = HEmpty<T>;
}

impl<T> Vars<T> for (T, T) {
    type Cons = HCons<T, HEmpty<T>>;
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

// impl <T> Deref for HEmpty<T> {
//     type Target = Option<T>;

//     fn deref(&self) -> &Self::Target {
//         &None
//     }
// }

// impl <T, E: HNext<T>> Deref for HCons<T, E> {
//     type Target = Option<T>;

//     fn deref(&self) -> &Self::Target {
//         &Some(self.0)
//     }
// }

impl<T, E: HNext<T>> HNext<T> for HCons<T, E> {
    type Next = E;

    fn value(&self) -> Option<&T> {
        Some(&self.0)
    }

    fn next(self) -> Self::Next {
        self.1
    }
}

#[test]
fn hcons_init() {
    let h = HCons(
        1,
        HCons(
            3,
            HEmpty::default()
        ),
    );

    assert_eq!(h.value(), Some(&1));

    let h1 = h.next();

    assert_eq!(h1.value(), Some(&3));

    let h2 = h1.next();

    assert_eq!(h2.value(), None);
}

#[test]
fn vars() {
    fn variadic(v: impl Vars<usize>) {
        let h = v.into_hcons();

        let v1 = h.value();

        let h2 = h.next();

        println!("{:?}", h2.value());

        let h3 = h2.next();

        println!("{:?}", h3.value());

        let h4 = h3.next();

        println!("{:?}", h4.value());
    }

    variadic((1, 2));
}

#[test]
fn explicit_vs_implicit_type() {
    let h = (1,2,).into_hcons();

    let n = h.value();
}
