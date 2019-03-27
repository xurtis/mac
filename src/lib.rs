//! Manadtory Access Control Monad

use std::marker::PhantomData;

/// Trait for items that mark an access level.
pub trait AccessLevel: 'static {}

/// Trait for showing relations between access levels.
pub trait Above<T: AccessLevel> {}

impl<T: AccessLevel> Above<T> for T {}

/// An object tagged with an access level.
///
/// Invofmation can only ever be passed to an access level of equal or higher value as desceibed by
/// the [`Above`](trait.Above.html) trait.
pub struct Mac<L, T> {
    value: T,
    level: PhantomData<L>,
}

impl<L: AccessLevel, T> Mac<L, T> {
    pub fn pure(value: T) -> Mac<L, T> {
        Mac {
            value,
            level: PhantomData,
        }
    }

    pub fn value(self) -> T {
        self.value
    }
}

impl <L: AccessLevel + 'static, A: 'static> Mac<L, A> {
    /// Map a value over an operation without an access level.
    pub fn map_unchecked<B>(self, f: impl Fn(A) -> B + 'static) -> Mac<L, B> {
        let Mac { value, level } = self;
        Mac {
            value: f(value),
            level,
        }
    }

    /// Map a value into a new access level.
    pub fn and_then<B, LL>(self, f: impl Fn(A) -> Mac<LL, B>) -> Mac<LL, B>
    where
        LL: AccessLevel + Above<L>,
    {
        f(self.value)
    }

    /// Map a value through an operation with an access level.
    pub fn map<B, LL>(self, f: Mac<LL, impl Fn(A) -> B>) -> Mac<LL, B>
    where
        LL: AccessLevel + Above<L>,
    {
        let Mac { value: operation, level } = f;
        Mac { value: operation(self.value), level }
    }
}
