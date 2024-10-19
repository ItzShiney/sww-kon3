use kon3::prelude::*;

struct Settings;
impl WindowSettings for Settings {
    fn window_attributes(&self) -> WindowAttributes {
        window_attributes("counter", 550, 310)
    }
}

fn main() {
    let counter = Shared::new(0_usize);

    let element = column((
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
    ));

    app(element, Settings).run().unwrap();
}
