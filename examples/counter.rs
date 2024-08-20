use kon3::prelude::*;

struct Counter;
impl Anchor for Counter {
    type Value = usize;
}

#[rustfmt::skip]
fn ui_builder() -> impl BuildElement {
    let counter_label = {
        label(concat((
            "clicked ",
            strfy(set::<Counter>(0)),
            " times",
        )))
    };

    let increase_button = {
        on_click(
            layers((
                rect(Color::GREEN),
                label("click me!"),
            )),
            write(get::<Counter>()),
            |counter| { *counter += 1; Consume },
        )
    };

    column((
        counter_label,
        increase_button,
    ))
}

fn main() {
    app::build(ui_builder()).run().unwrap();
}
