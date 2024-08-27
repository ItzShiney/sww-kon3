use kon3::prelude::*;

#[rustfmt::skip]
fn element_builder(app: &SharedBuilder) -> impl Element<Resources> {
    let counter = app.shared(0_usize);

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
            move || { *counter.lock() += 1; Consume }
        )
    };

    column((
        counter_label,
        increase_button,
    ))
}

fn main() {
    app::build(element_builder).run().unwrap();
}
