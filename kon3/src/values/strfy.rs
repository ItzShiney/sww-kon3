use super::Cache;
use super::CacheGuard;
use super::ValueSource;
use crate::shared;
use crate::InvalidateCaches;
use std::collections::BTreeSet;
use std::ops::Deref;

pub struct Strfy<Src> {
    source: Src,
    cache: Cache<String>,
}

impl<Src: for<'s> ValueSource<Value<'s>: Deref<Target: ToString>>> ValueSource for Strfy<Src> {
    type Value<'s> = CacheGuard<'s, String> where Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self.cache
            .get_or_insert_with(|| self.source.value().to_string())
    }
}

impl<Src: InvalidateCaches> InvalidateCaches for Strfy<Src> {
    fn invalidate_caches(&self, addrs: &BTreeSet<shared::Addr>) -> bool {
        if self.source.invalidate_caches(addrs) {
            self.cache.reset();
            true
        } else {
            false
        }
    }
}

pub const fn strfy<Src: for<'s> ValueSource<Value<'s>: Deref<Target: ToString>>>(
    ra_fixture_source: Src,
) -> Strfy<Src> {
    Strfy {
        source: ra_fixture_source,
        cache: Cache::new(),
    }
}
