use kon3::prelude::*;

struct Counter;
impl Anchor for Counter {
    type Value = usize;
}

#[rustfmt::skip]
fn ui_builder() -> impl BuildElement {
    let counter = {
        label(concat((
            "clicked ",
            strfy(set::<Counter>(0)),
            " times",
        )))
    };

    let button = {
        on_click(
            layers((
                label("click me!"),
                rect(Color::GREEN),
            )),
            write(get::<Counter>()),
            |counter| { *counter += 1; Consume },
        )
    };

    column((
        counter,
        button,
    ))
}

fn main() {
    app::build(ui_builder()).run().unwrap();
}
