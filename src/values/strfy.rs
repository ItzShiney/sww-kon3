use super::SourcedValue;
use super::ValueSource;
use crate::cache;
use crate::shared::Shared;
use crate::Anchor;
use crate::Build;
use crate::Cache;
use crate::Cached;
use crate::ResolveAnchors;

#[derive(Debug)]
pub struct Strfy<Src, Cch> {
    source: Src,
    cache: Cch,
}

impl<Src: Build, Cch: Build> Build for Strfy<Src, Cch> {
    type Built = Strfy<Src::Built, Cch::Built>;

    fn build(self) -> Self::Built {
        Strfy {
            source: self.source.build(),
            cache: self.cache.build(),
        }
    }
}

impl<Src: ResolveAnchors, Cch> ResolveAnchors for Strfy<Src, Cch> {
    type AnchorsSet = Src::AnchorsSet;

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        self.source.get_anchor::<A>()
    }

    fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
        self.source.resolve_anchor::<A>(anchor);
    }
}

// FIXME: `+ std::fmt::Debug`
impl<Src: ValueSource<Value: ToString + std::fmt::Debug>> ValueSource
    for Strfy<Src, Cached<String>>
{
    type Value = str;

    fn value(&self) -> SourcedValue<'_, Self::Value> {
        let mut value = self.cache.borrow_mut();
        *value = None; // FIXME
        value.get_or_insert_with(|| self.source.value().to_string());
        SourcedValue::Cached(value)
    }
}

pub fn strfy<V: ToString, Src: Build<Built: ValueSource<Value = V>>>(
    ra_fixture_source: Src,
) -> Strfy<Src, Cache<String>> {
    Strfy {
        source: ra_fixture_source,
        cache: cache(),
    }
}
