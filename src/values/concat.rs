use super::Cache;
use super::SourcedValue;
use super::ValueSource;

pub struct Concat<Src> {
    sources: Src,
    cache: Cache<String>,
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
