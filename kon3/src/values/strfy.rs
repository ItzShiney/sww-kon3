use super::Cache;
use super::CacheGuard;
use super::ContainsShared;
use super::ValueSource;
use crate::app::SignalSender;
use crate::shared::SharedAddr;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use std::ops::Deref;

pub struct Strfy<Src> {
    source: Src,
    cache: Cache<String>,
}

impl<Src: for<'s> ValueSource<Value<'s>: Deref<Target: ToString>>> ValueSource for Strfy<Src> {
    type Value<'s>
        = CacheGuard<'s, String>
    where
        Self: 's;

    fn value(&self) -> Self::Value<'_> {
        self.cache
            .get_or_insert_with(|| self.source.value().to_string())
    }
}

impl<Src: ContainsShared> ContainsShared for Strfy<Src> {
    fn contains_shared(&self, addr: SharedAddr) -> bool {
        self.source.contains_shared(addr)
    }
}

impl<Src: HandleEvent> HandleEvent for Strfy<Src> {
    fn handle_event(&self, signal_sender: &SignalSender, event: &Event) -> EventResult {
        self.cache.reset();
        self.source.handle_event(signal_sender, event)
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
