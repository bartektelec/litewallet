use seed::prelude::*;
use seed::*;

pub const fn init() -> Model {
    "".to_string()
}

pub type Model = String;

#[derive(Clone, Copy)]
pub enum Msg {
    Change(text),
}

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Change(text) => {
            model.text = text;
        }
    }
}

pub fn view(model: Model) -> Node<Msg> {
    div![
        input![
            attrs! {
            At::Type => "text"
            },
            ev(Ev::Input, |e| Msg::Change(e)),
        ],
        button![ev(Ev::Click, |_| Msg::Dec), "-"],
        div![model],
        button![ev(Ev::Click, |_| Msg::Inc), "+"]
    ]
}
