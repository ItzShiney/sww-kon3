use crate as kon3;
use crate::consume;
use crate::id;
use crate::shared::Shared;
use crate::values::ArgsSource;
use crate::values::ValueSource;
use crate::Anchor;
use crate::Build;
use crate::BuildElement;
use crate::Element;
use crate::Event;
use crate::EventConsumed;
use crate::HandleEvent;
use crate::Ident;
use crate::ResolveAnchors;
use std::fmt;
use std::fmt::Debug;
use sww::Color;

#[derive(Debug)]
pub struct Rect(pub Color);

impl Build for Rect {
    type Output = Self;

    fn build(self) -> Self::Output {
        self
    }
}

impl ResolveAnchors for Rect {
    type AnchorsSet = ();

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        None
    }

    fn resolve_anchor<A: Anchor>(&mut self, _anchor: &Shared<A::Value>) {}
}

impl Element for Rect {}

impl HandleEvent for Rect {
    fn handle_event(&mut self, _event: &Event) -> Result<(), EventConsumed> {
        Ok(())
    }
}

pub const fn rect(ra_fixture_color: Color) -> Rect {
    Rect(ra_fixture_color)
}

#[derive(Debug)]
pub enum SplitType {
    Vertical,
    Horizontal,
    Adaptive,
}

#[derive(Debug)]
pub struct Split<Ty, Es> {
    type_: Ty,
    elements: Es,
}

impl<Ty: Build, Es: Build> Build for Split<Ty, Es> {
    type Output = Split<Ty::Output, Es::Output>;

    fn build(self) -> Self::Output {
        Split {
            type_: self.type_.build(),
            elements: self.elements.build(),
        }
    }
}

impl<Ty: ResolveAnchors, Es: ResolveAnchors> ResolveAnchors for Split<Ty, Es> {
    type AnchorsSet = (Ty::AnchorsSet, Es::AnchorsSet);

    fn get_anchor<_A: Anchor>(&self) -> Option<Shared<_A::Value>> {
        (self.type_.get_anchor::<_A>()).or_else(|| self.elements.get_anchor::<_A>())
    }

    fn resolve_anchor<_A: Anchor>(&mut self, anchor: &Shared<_A::Value>) {
        self.type_.resolve_anchor::<_A>(anchor);
        self.elements.resolve_anchor::<_A>(anchor);
    }
}

impl<Ty, A: Element, B: Element> Element for Split<Ty, (A, B)> {}

impl<Ty, Es: HandleEvent> HandleEvent for Split<Ty, Es> {
    fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
        self.elements.handle_event(event)
    }
}

pub const fn split<Es: Build>(ra_fixture_elements: Es) -> Split<SplitType, Es> {
    Split {
        type_: SplitType::Adaptive,
        elements: ra_fixture_elements,
    }
}

pub const fn line<Es: Build>(ra_fixture_elements: Es) -> Split<Ident<SplitType>, Es> {
    Split {
        type_: id(SplitType::Horizontal),
        elements: ra_fixture_elements,
    }
}

pub fn column<Es: Build>(ra_fixture_elements: Es) -> Split<Ident<SplitType>, Es> {
    Split {
        type_: id(SplitType::Vertical),
        elements: ra_fixture_elements,
    }
}

#[derive(Debug, Build)]
pub struct Layers<Es>(Es);

impl<A: Element, B: Element> Element for Layers<(A, B)> {}
impl<A: Element, B: Element, C: Element> Element for Layers<(A, B, C)> {}

impl<Es: HandleEvent> HandleEvent for Layers<Es> {
    fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
        self.0.handle_event(event)
    }
}

pub const fn layers<Es>(ra_fixture_elements: Es) -> Layers<Es> {
    Layers(ra_fixture_elements)
}

#[derive(Debug, Build)]
pub struct Label<Src>(Src);

impl<Src> HandleEvent for Label<Src> {
    fn handle_event(&mut self, _event: &Event) -> Result<(), EventConsumed> {
        Ok(())
    }
}

impl<Src: ValueSource<Value = str>> Element for Label<Src> {}

pub const fn label<Src: Build<Output: ValueSource<Value = str>>>(
    ra_fixture_source: Src,
) -> Label<Src> {
    Label(ra_fixture_source)
}

pub struct OnClickConsume<E, Src, F> {
    element: E,
    source: Src,
    f: F,
}

impl<E: Debug, Src: Debug, F> Debug for OnClickConsume<E, Src, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OnClickConsume")
            .field("element", &self.element)
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}

impl<E: Build, Src: Build, F> Build for OnClickConsume<E, Src, F> {
    type Output = OnClickConsume<E::Output, Src::Output, F>;

    fn build(self) -> Self::Output {
        OnClickConsume {
            element: self.element.build(),
            source: self.source.build(),
            f: self.f,
        }
    }
}

impl<E: ResolveAnchors, Src: ResolveAnchors, F> ResolveAnchors for OnClickConsume<E, Src, F> {
    type AnchorsSet = (E::AnchorsSet, Src::AnchorsSet);

    fn get_anchor<A: Anchor>(&self) -> Option<Shared<A::Value>> {
        (self.element.get_anchor::<A>()).or_else(|| self.source.get_anchor::<A>())
    }

    fn resolve_anchor<A: Anchor>(&mut self, anchor: &Shared<A::Value>) {
        self.element.resolve_anchor::<A>(anchor);
        self.source.resolve_anchor::<A>(anchor);
    }
}

// impl<E: Element, Src, F> Element for OnClickConsume<E, Src, F> where Self: HandleEvent {}
impl<E: Element, Src: ArgsSource> Element for OnClickConsume<E, Src, Src::Fn> {}

impl<E: HandleEvent, Src: ArgsSource> HandleEvent for OnClickConsume<E, Src, Src::Fn> {
    fn handle_event(&mut self, event: &Event) -> Result<(), EventConsumed> {
        match event {
            Event::Click => {
                self.source.apply_to(&self.f);
                consume()
            }

            _ => self.element.handle_event(event),
        }
    }
}

pub const fn on_click_consume<E: BuildElement, Src: Build<Output = ArgSrc>, ArgSrc: ArgsSource>(
    ra_fixture_element: E,
    ra_fixture_source: Src,
    ra_fixture_f: ArgSrc::Fn,
) -> OnClickConsume<E, Src, ArgSrc::Fn> {
    OnClickConsume {
        element: ra_fixture_element,
        source: ra_fixture_source,
        f: ra_fixture_f,
    }
}
