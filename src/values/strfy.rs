use super::Cache;
use super::SourcedValue;
use super::ValueSource;
use crate::shared::Shared;
use crate::Anchor;
use crate::Build;
use crate::ResolveAnchors;

pub struct Strfy<Src> {
    source: Src,
    cache: Cache<String>,
}

impl<Src: Build> Build for Strfy<Src> {
    type Built = Strfy<Src::Built>;

    fn build(self) -> Self::Built {
        Strfy {
            source: self.source.build(),
            cache: self.cache,
        }
    }
}

impl<Src: ResolveAnchors> ResolveAnchors for Strfy<Src> {
    type AnchorsSet = Src::AnchorsSet;

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        self.source.get_anchor::<A>()
    }

    fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
        self.source.resolve_anchor::<A>(anchor);
    }
}

// FIXME: `+ std::fmt::Debug`
impl<Src: ValueSource<Value: ToString + std::fmt::Debug>> ValueSource for Strfy<Src> {
    type Value = str;

    fn value(&self) -> SourcedValue<'_, Self::Value> {
        SourcedValue::Cached(
            self.cache
                .get_or_insert_with(|| self.source.value().to_string()),
        )
    }
}

pub fn strfy<V: ToString, Src: Build<Built: ValueSource<Value = V>>>(
    ra_fixture_source: Src,
) -> Strfy<Src> {
    Strfy {
        source: ra_fixture_source,
        cache: Cache::new(),
    }
}
