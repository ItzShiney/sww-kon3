enum Inner<T, B> {
    Builder(Option<B>),
    Value(T),
}

pub struct Lazy<T, B>(Inner<T, B>);

impl<T, B> Lazy<T, B> {
    pub fn new(builder: B) -> Self {
        Self(Inner::Builder(Some(builder)))
    }

    pub fn get(&self) -> Option<&T> {
        match &self.0 {
            Inner::Builder(_) => None,
            Inner::Value(value) => Some(value),
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        match &mut self.0 {
            Inner::Builder(_) => None,
            Inner::Value(value) => Some(value),
        }
    }

    pub fn get_or_init<In>(&mut self, arg: In) -> &mut T
    where
        B: FnOnce(In) -> T,
    {
        self.get_or_init_map(arg, |value| value)
    }

    pub fn get_or_init_map<In, R>(&mut self, arg: In, map: impl FnOnce(R) -> T) -> &mut T
    where
        B: FnOnce(In) -> R,
    {
        if let Inner::Builder(builder) = &mut self.0 {
            let f = builder.take().unwrap();
            self.0 = Inner::Value(map(f(arg)));
        }

        let Inner::Value(value) = &mut self.0 else {
            unreachable!();
        };
        value
    }
}
