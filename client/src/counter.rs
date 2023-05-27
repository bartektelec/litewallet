use seed::prelude::*;
use seed::*;

pub const fn init() -> Model {
    0
}

pub type Model = i32;

#[derive(Clone, Copy)]
pub enum Msg {
    Inc,
    Dec,
}

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::Inc => *model += 1,
        Msg::Dec => *model -= 1,
    }
}

pub fn view(model: Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::Dec), "-"],
        div![model],
        button![ev(Ev::Click, |_| Msg::Inc), "+"]
    ]
}
