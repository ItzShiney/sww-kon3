use kon::prelude::*;

struct Counter;
pub type CounterValue = usize;
impl Anchor for Counter {
    type Value = CounterValue;
}

#[rustfmt::skip]
fn make_button() -> impl BuildElement {
    fn increase_counter(counter: &mut CounterValue) {
        *counter += 1;
    }

    on_click_consume(
        layers((
            label(id("click me!")),
            fill(Color::GREEN),
        )),
        get::<Counter>(),
        increase_counter,
    )
}

#[rustfmt::skip]
fn make_ui() -> impl BuildElement {
    column((
        label(concat((
            id("clicked "),
            strfy::<CounterValue, _>(set::<Counter>(0)),
            id(" times"),
        ))),
        make_button(),
    ))
}

fn main() {
    let mut ui = build(make_ui());

    println!("{:#?}", ui);
    _ = ui.handle_event(&Event::Click);
    println!("{:#?}", ui);
}
