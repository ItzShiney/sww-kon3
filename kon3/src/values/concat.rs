use super::Cache;
use super::CacheRef;
use super::ValueSource;
use super::ValueSourceBorrow;
use crate::shared;
use crate::InvalidateCache;
use std::borrow::Borrow;

pub struct Concat<Src> {
    sources: Src,
    cache: Cache<String>,
}

impl<A: ValueSourceBorrow<str>, B: ValueSourceBorrow<str>, C: ValueSourceBorrow<str>> ValueSource
    for Concat<(A, B, C)>
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

impl<Src: InvalidateCache> InvalidateCache for Concat<Src> {
    fn invalidate_cache(&self, addr: shared::Addr) -> bool {
        if self.sources.invalidate_cache(addr) {
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
