use kon3::prelude::*;

fn element_builder() -> impl Element {
    let counter = Shared::new(0_usize);

    column((
        '_counter: { label(concat(("clicked ", strfy(counter.clone()), " times"))) },
        '_button: {
            on_click(
                layers((rect(Color::GREEN), label("click me!"))),
                move |signal_sender| {
                    *counter.write(signal_sender) += 1;
                    Consume
                },
            )
        },
    ))
}

fn main() {
    app::run(element_builder()).unwrap();
}
