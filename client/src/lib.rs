// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod counter;
mod input;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counters: (0..3).map(|_| counter::init()).collect(),
        text: "".to_string(),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counters: Vec<counter::Model>,
    text: String,
}

// ------ ------
//    Update
// ------ ------
//
type CounterId = usize;

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
enum Msg {
    Counter(counter::Msg, CounterId),
    InputText(String),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Counter(msg, id) => counter::update(msg, &mut model.counters[id]),
        Msg::InputText(in_text) => model.text = in_text,
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        div![&model.text],
        input::view("".to_string()),
        model.counters.iter().enumerate().map(|(id, model)| {
            counter::view(*model).map_msg(move |counter_msg| Msg::Counter(counter_msg, id))
        })
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
