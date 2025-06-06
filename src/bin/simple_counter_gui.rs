use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}


fn app() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count +=1, "Up high!"}
        button { onclick: move |_| count -=1, "Dow low!"}
    }    
}
