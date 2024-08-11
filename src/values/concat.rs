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
pub struct Concat<Src, Cch> {
    sources: Src,
    cache: Cch,
}

impl<Src: Build, Cch: Build> Build for Concat<Src, Cch> {
    type Built = Concat<Src::Built, Cch::Built>;

    fn build(self) -> Self::Built {
        Concat {
            sources: self.sources.build(),
            cache: self.cache.build(),
        }
    }
}

impl<Src: ResolveAnchors, Cch> ResolveAnchors for Concat<Src, Cch> {
    type AnchorsSet = Src::AnchorsSet;

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        self.sources.get_anchor::<A>()
    }

    fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
        self.sources.resolve_anchor::<A>(anchor);
    }
}

impl<A: ValueSource<Value = str>, B: ValueSource<Value = str>, C: ValueSource<Value = str>>
    ValueSource for Concat<(A, B, C), Cached<String>>
{
    type Value = str;

    fn value(&self) -> SourcedValue<'_, Self::Value> {
        todo!()
    }
}

pub const fn concat<Srcs>(ra_fixture_sources: Srcs) -> Concat<Srcs, Cache<String>> {
    Concat {
        sources: ra_fixture_sources,
        cache: cache(),
    }
}
