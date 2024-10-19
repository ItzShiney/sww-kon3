use super::Cache;
use super::CacheGuard;
use super::ValueSource;
use super::ValueSourceBorrow;
use crate::app::Signaler;
use crate::shared::SharedAddr;
use crate::ContainsShared;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use std::borrow::Borrow;

pub struct Concat<Src> {
    sources: Src,
    cache: Cache<String>,
}

impl<A: ValueSourceBorrow<str>, B: ValueSourceBorrow<str>, C: ValueSourceBorrow<str>> ValueSource
    for Concat<(A, B, C)>
{
    type Value<'s>
        = CacheGuard<'s, String>
    where
        Self: 's;

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

impl<Src: ContainsShared> ContainsShared for Concat<Src> {
    fn contains_shared(&self, addr: SharedAddr) -> bool {
        self.sources.contains_shared(addr)
    }
}

impl<Src: HandleEvent> HandleEvent for Concat<Src> {
    fn handle_event(&self, signaler: &Signaler, event: &Event) -> EventResult {
        self.cache.reset();
        self.sources.handle_event(signaler, event)
    }
}

pub const fn concat<Srcs>(ra_fixture_sources: Srcs) -> Concat<Srcs> {
    Concat {
        sources: ra_fixture_sources,
        cache: Cache::new(),
    }
}
