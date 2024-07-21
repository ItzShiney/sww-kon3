mod inner;

use inner::*;

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
