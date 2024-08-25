use app::Resources;
use kon3::prelude::*;

#[rustfmt::skip]
fn ui_builder() -> impl Element<Resources> {
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
            {
                #[allow(clippy::redundant_clone)]
                let counter = counter.clone();
                move || { *counter.lock() += 1; Consume }
            },
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
