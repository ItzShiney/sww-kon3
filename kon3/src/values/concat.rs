use super::Cache;
use super::CacheGuard;
use super::ValueSource;
use super::ValueSourceBorrow;
use crate::shared;
use crate::InvalidateCaches;
use std::borrow::Borrow;
use std::collections::BTreeSet;

pub struct Concat<Src> {
    sources: Src,
    cache: Cache<String>,
}

impl<A: ValueSourceBorrow<str>, B: ValueSourceBorrow<str>, C: ValueSourceBorrow<str>> ValueSource
    for Concat<(A, B, C)>
{
    type Value<'s> = CacheGuard<'s, String> where Self: 's;

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

impl<Src: InvalidateCaches> InvalidateCaches for Concat<Src> {
    fn invalidate_caches(&self, addrs: &BTreeSet<shared::Addr>) -> bool {
        if self.sources.invalidate_caches(addrs) {
            self.cache.reset();
            true
        } else {
            false
        }
    }
}

pub const fn concat<Srcs>(ra_fixture_sources: Srcs) -> Concat<Srcs> {
    Concat {
        sources: ra_fixture_sources,
        cache: Cache::new(),
    }
}
