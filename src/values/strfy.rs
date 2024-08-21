use super::Cache;
use super::SourcedValue;
use super::ValueSource;

pub struct Strfy<Src> {
    source: Src,
    cache: Cache<String>,
}

impl<Src: ValueSource<Value: ToString>> ValueSource for Strfy<Src> {
    type Value = str;

    fn value(&self) -> SourcedValue<'_, Self::Value> {
        SourcedValue::Cached(
            self.cache
                .get_or_insert_with(|| self.source.value().to_string()),
        )
    }
}

pub fn strfy<V: ToString, Src: ValueSource<Value = V>>(ra_fixture_source: Src) -> Strfy<Src> {
    Strfy {
        source: ra_fixture_source,
        cache: Cache::new(),
    }
}
