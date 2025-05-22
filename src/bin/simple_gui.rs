use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx!("Hello World!")
}
