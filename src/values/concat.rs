use super::Cache;
use super::CacheRef;
use super::ValueSource;
use std::borrow::Borrow;
use std::ops::Deref;

pub struct Concat<Src> {
    sources: Src,
    cache: Cache<String>,
}

impl<
        A: for<'s> ValueSource<Value<'s>: Deref<Target: Borrow<str>>>,
        B: for<'s> ValueSource<Value<'s>: Deref<Target: Borrow<str>>>,
        C: for<'s> ValueSource<Value<'s>: Deref<Target: Borrow<str>>>,
    > ValueSource for Concat<(A, B, C)>
{
    type Value<'s> = CacheRef<'s, String> where Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self.cache.get_or_insert_with(|| {
            format!(
                "{}{}{}",
                (*self.sources.0.value()).borrow(),
                (*self.sources.1.value()).borrow(),
                (*self.sources.2.value()).borrow(),
            )
        })
    }
}

pub const fn concat<Srcs>(ra_fixture_sources: Srcs) -> Concat<Srcs> {
    Concat {
        sources: ra_fixture_sources,
        cache: Cache::new(),
    }
}
