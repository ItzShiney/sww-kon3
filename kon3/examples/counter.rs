use kon3::prelude::*;

fn element_builder(signal_sender: &SignalSender) -> impl Element {
    let counter = signal_sender.shared(0_usize);

    column((
        '_counter: { label(concat(("clicked ", strfy(counter.clone()), " times"))) },
        '_button: {
            on_click(
                layers((rect(Color::GREEN), label("click me!"))),
                consume(move || *counter.lock() += 1),
            )
        },
    ))
}

fn main() {
    app::run(element_builder).unwrap();
}
