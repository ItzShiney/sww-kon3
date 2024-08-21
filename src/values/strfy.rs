use super::Cache;
use super::CacheRef;
use super::ValueSource;
use std::ops::Deref;

pub struct Strfy<Src> {
    source: Src,
    cache: Cache<String>,
}

impl<Src: for<'s> ValueSource<Value<'s>: Deref<Target: ToString>>> ValueSource for Strfy<Src> {
    type Value<'s> = CacheRef<'s, String> where Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self.cache
            .get_or_insert_with(|| self.source.value().to_string())
    }
}

pub fn strfy<Src: for<'s> ValueSource<Value<'s>: Deref<Target: ToString>>>(
    ra_fixture_source: Src,
) -> Strfy<Src> {
    Strfy {
        source: ra_fixture_source,
        cache: Cache::new(),
    }
}
