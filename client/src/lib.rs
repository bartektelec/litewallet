// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use reqwest;
use seed::{prelude::*, *};
mod counter;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counters: (0..3).map(|_| counter::init()).collect(),
        text: "".to_string(),
        login_form: LoginForm {
            username: String::new(),
            password: String::new(),
        },
        user_data: None,
    }
}

// ------ ------
//     Model
// ------ ------

struct LoginForm {
    username: String,
    password: String,
}

struct Account {
    account_number: String,
    balance: String,
}

struct User {
    username: String,
    id: i32,
    accounts: Vec<Account>,
}

struct Model {
    counters: Vec<counter::Model>,
    text: String,
    user_data: Option<User>,
    login_form: LoginForm,
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
    SubmitLogin,
    Logout,
}

fn login_form(model: &Model) -> Node<Msg> {
    if let Some(user) = &model.user_data {
        return div![
            format!("You are logged in as {}", user.username),
            button!["Log out", ev(Ev::Click, |_| Msg::Logout)]
        ];
    } else {
        div![
            label![
                plain!["Username"],
                input![attrs![
                    At::Type => "text"
                ]]
            ],
            label![
                plain!["Password"],
                input![attrs![
                    At::Type => "password"
                ]]
            ],
            button![ev(Ev::Click, |_| Msg::SubmitLogin), "Login"]
        ]
    }
}

fn transfer_box(user_data: &User) -> Node<Msg> {
    div![
        h2!["Transfer"],
        label![
            "Transfer from account",
            select![user_data
                .accounts
                .iter()
                .map(|acc| { option![format!("{} : ${}", acc.account_number, acc.balance)] })]
        ],
        label![
            "Transfer to account",
            input![attrs![
                At::Type => "number"
            ],]
        ],
        button!["Send"]
    ]
}

fn account_history(user: &User) -> Node<Msg> {
    div![
        h2!["Account history"],
        ul![
            li!["Sent $20 to 10202321321"],
            li!["Received $50 from 12313213231"]
        ]
    ]
}

fn app_body(model: &Model) -> Node<Msg> {
    if let Some(user) = &model.user_data {
        return div![
            transfer_box(user),
            h1!["Here are your accounts:"],
            ul![user
                .accounts
                .iter()
                .map(|acc| { li![format!("{} --- {}", acc.account_number, acc.balance)] })],
            account_history(user)
        ];
    } else {
        empty![]
    }
}

fn perform_login_user(model: &mut Model) {
    model.user_data = Some(User {
        id: 1,
        username: "bartek".to_string(),
        accounts: vec![Account {
            account_number: "10001123321".to_string(),
            balance: "20".to_string(),
        }],
    })
}

fn perform_logout(model: &mut Model) {
    model.user_data = None
}
// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Counter(msg, id) => counter::update(msg, &mut model.counters[id]),
        Msg::InputText(in_text) => model.text = in_text,
        Msg::SubmitLogin => perform_login_user(model),
        Msg::Logout => perform_logout(model),
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![login_form(model), app_body(model)]
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
