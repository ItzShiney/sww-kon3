use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Mutex;
use std::sync::MutexGuard;

enum Inner<T, B> {
    Builder(Option<B>),
    Value(T),
}

pub struct Lazy<T, B>(Mutex<Inner<T, B>>);

impl<T, B> Lazy<T, B> {
    pub fn new(builder: B) -> Self {
        Self(Mutex::new(Inner::Builder(Some(builder))))
    }

    pub fn get(&self) -> Option<LazyGuard<'_, T, B>> {
        let guard = self.0.lock().unwrap();

        if matches!(&*guard, Inner::Value(_)) {
            Some(LazyGuard(guard))
        } else {
            None
        }
    }

    pub fn get_or_init<In>(&self, arg: In) -> LazyGuard<'_, T, B>
    where
        B: FnOnce(In) -> T,
    {
        self.get_or_init_map(arg, |value| value)
    }

    pub fn get_or_init_map<In, R>(&self, arg: In, map: impl FnOnce(R) -> T) -> LazyGuard<'_, T, B>
    where
        B: FnOnce(In) -> R,
    {
        let mut guard = self.0.lock().unwrap();

        if let Inner::Builder(builder) = &mut *guard {
            let f = builder.take().unwrap();
            *guard = Inner::Value(map(f(arg)));
        }

        LazyGuard(guard)
    }
}

pub struct LazyGuard<'s, T, B>(MutexGuard<'s, Inner<T, B>>);

impl<T, B> Deref for LazyGuard<'_, T, B> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &*self.0 {
            Inner::Builder(_) => unreachable!(),
            Inner::Value(value) => value,
        }
    }
}

impl<T, B> DerefMut for LazyGuard<'_, T, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut *self.0 {
            Inner::Builder(_) => unreachable!(),
            Inner::Value(value) => value,
        }
    }
}
