use std::hint::unreachable_unchecked;
use std::mem::ManuallyDrop;

pub struct Lazy<T, F>(Inner<T, F>);

impl<T, F> Lazy<T, F> {
    pub fn new(f: F) -> Self {
        Self(Inner::new(f))
    }

    pub fn value<Arg>(&mut self, arg: Arg) -> &mut T
    where
        F: FnOnce(Arg) -> T,
    {
        self.0.value(arg)
    }
}

enum Inner<T, F> {
    Lazy(ManuallyDrop<F>),
    Value(T),
}

impl<T, F> Inner<T, F> {
    fn new(f: F) -> Self {
        Self::Lazy(ManuallyDrop::new(f))
    }

    fn value<Arg>(&mut self, arg: Arg) -> &mut T
    where
        F: FnOnce(Arg) -> T,
    {
        if let Self::Lazy(f) = self {
            let f = unsafe { ManuallyDrop::take(f) };
            *self = Self::Value(f(arg));
        }

        let Self::Value(value) = self else {
            unsafe { unreachable_unchecked() }
        };

        value
    }
}

impl<T, F> Drop for Inner<T, F> {
    fn drop(&mut self) {
        if let Self::Lazy(f) = self {
            _ = unsafe { ManuallyDrop::take(f) };
        }
    }
}
