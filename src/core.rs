
pub trait Vars<Item> {
    
}

pub trait Tuple {}
pub trait TupleG<E> {}

pub trait UnCons {
    type Head;
    type Tail;
    type Output;

    fn uncons(self) -> Self::Output;
}

pub trait PushFront<E> {
    type Output;

    fn push_front(self, e: E) -> Self::Output;
}

pub trait Compose<Init> {
    type Output;

    fn compose(self, init: Init) -> Self::Output;
}

impl <A,B, X,Y,Z> Compose<X> for (A, B)
where
    A: Fn(X) -> Y,
    B: Fn(Y) -> Z, {
    type Output = Z;

    fn compose(self, init: X) -> Self::Output {
        (self.1)((self.0)(init))
    }
}