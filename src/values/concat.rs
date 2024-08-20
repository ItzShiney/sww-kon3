use super::Cache;
use super::SourcedValue;
use super::ValueSource;
use crate::shared::Shared;
use crate::Anchor;
use crate::Build;
use crate::ResolveAnchors;

pub struct Concat<Src> {
    sources: Src,
    cache: Cache<String>,
}

impl<Src: Build> Build for Concat<Src> {
    type Built = Concat<Src::Built>;

    fn build(self) -> Self::Built {
        Concat {
            sources: self.sources.build(),
            cache: self.cache,
        }
    }
}

impl<Src: ResolveAnchors> ResolveAnchors for Concat<Src> {
    type AnchorsSet = Src::AnchorsSet;

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        self.sources.get_anchor::<A>()
    }

    fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
        self.sources.resolve_anchor::<A>(anchor);
    }
}

impl<A: ValueSource<Value = str>, B: ValueSource<Value = str>, C: ValueSource<Value = str>>
    ValueSource for Concat<(A, B, C)>
{
    type Value = str;

    fn value(&self) -> SourcedValue<'_, Self::Value> {
        SourcedValue::Cached(self.cache.get_or_insert_with(|| {
            format!(
                "{}{}{}",
                &*self.sources.0.value(),
                &*self.sources.1.value(),
                &*self.sources.2.value(),
            )
        }))
    }
}

pub const fn concat<Srcs>(ra_fixture_sources: Srcs) -> Concat<Srcs> {
    Concat {
        sources: ra_fixture_sources,
        cache: Cache::new(),
    }
}
