use app::Resources;
use kon3::prelude::*;
use std::sync::Arc;

#[rustfmt::skip]
fn ui_builder() -> Arc<dyn Element<Resources>> {
    Arc::new_cyclic(|ui| {
        let counter = ui.new_shared(0_usize);

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
    })
}

fn main() {
    app::build(ui_builder()).run().unwrap();
}
