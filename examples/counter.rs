use kon3::prelude::*;

#[rustfmt::skip]
fn ui_builder() -> impl Element {
    let counter = Shared::new(0_usize);

    let counter_label = {
        label(concat((
            "clicked ",
            strfy(counter.clone()),
            " times",
        )))
    };

    let increase_button = {
        on_click(
            layers((
                rect(Color::GREEN),
                label("click me!"),
            )),
            write(counter.clone()),
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
