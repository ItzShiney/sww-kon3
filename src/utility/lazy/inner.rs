pub enum Inner<T, F> {
    Lazy(Option<F>),
    Value(T),
}

impl<T, F> Inner<T, F> {
    pub fn new(f: F) -> Self {
        Self::Lazy(Some(f))
    }

    pub fn value<Arg>(&mut self, arg: Arg) -> &mut T
    where
        F: FnOnce(Arg) -> T,
    {
        if let Self::Lazy(f) = self {
            let f = f.take().unwrap();
            *self = Self::Value(f(arg));
        }

        let Self::Value(value) = self else {
            unreachable!()
        };

        value
    }
}
