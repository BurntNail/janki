///Enum which can represent one of two values
///
///The Same as an `(Option<A>, Option<B>)` where one [`Option`] must always be [`Option::Some`] and the other must be [`Option::None`]
pub enum Either<A, B> {
    ///The First variant of [`Either`]
    Left(A),
    ///The second variant of [`Either`]
    Right(B),
}

impl<A, B> Either<A, B> {
    ///Constructor for [`Either::Left`] which uses [`Into::into`]
    pub fn l(a: impl Into<A>) -> Self {
        Self::Left(a.into())
    }

    ///Constructor for [`Either::Right`] which uses [`Into::into`]
    pub fn r(b: impl Into<B>) -> Self {
        Self::Right(b.into())
    }
}

impl<A> Either<A, A> {
    ///If `A` == `B` then this function will return an `A` - useful for when the [`Either`] side signifies something, but always returns the same type.
    #[allow(clippy::missing_const_for_fn)] //Cannot be const as destructors cannot be const - Github error 8874
    pub fn to_normal(self) -> A {
        match self {
            Self::Left(a) => a,
            Self::Right(b) => b,
        }
    }
}
