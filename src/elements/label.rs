use crate as kon3;
use crate::values::ValueSource;
use crate::Build;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;

#[derive(Build)]
pub struct Label<Src>(Src);

impl<Src> HandleEvent for Label<Src> {
    fn handle_event(&mut self, _event: &Event) -> EventResult {
        Ok(())
    }
}

impl<Src: ValueSource<Value = str>> Element for Label<Src> {}

pub const fn label<Src: Build<Built: ValueSource<Value = str>>>(
    ra_fixture_source: Src,
) -> Label<Src> {
    Label(ra_fixture_source)
}
