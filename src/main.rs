use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

#[component(App<G>)]
fn app() -> Template<G> {
    let name = Signal::new(String::new());

    let displayed_name = cloned!((name) => move || {
        if name.get().is_empty() {
            "World".to_string()
        } else {
            name.get().as_ref().clone()
        }
    });

    let handle_change = move |event: Event| {
        name.set(
            event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value(),
        );
    };

    template! {
        div {
            h1 {
                "Hello "
                (displayed_name())
                "!"
            }

            input(placeholder="What is your name?", on:input=handle_change)
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|| template! { App() });
}
