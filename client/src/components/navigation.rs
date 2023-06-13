use leptos::*;
use reqwasm::http::*;

use crate::components::modal::*;

#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    let is_login_open = create_rw_signal(cx, false);
    let is_register_open = create_rw_signal(cx, false);

    let open_login = move || {
        is_login_open.set(true);
    };

    let open_register = move || {
        is_register_open.set(true);
    };

    let run_req = move |_| {
        let resp = Request::get("https://swapi.dev/api/people/")
            .send()
            .await
            .unwrap();

        log!("{}", resp.text().await.unwrap());
    };

    view! {cx,
    <nav class="bg-gray-800 py-3 text-white">
        <div class="container flex justify-between items-center mx-auto">

        <a href="/" class="font-semibold text-lg">"Litewallet"</a>

        <div class="flex gap-2 items-center">
            <button on:click=move |_| { open_register(); } class="btn btn-secondary">"Sign up"</button>
            <button on:click=move |_| { open_login(); } class="btn btn-primary">"Log in"</button>
        </div>
        </div>
    </nav>
    <Modal is_open=is_login_open>
        "Login modal"
    </Modal>
    <Modal is_open=is_register_open>
        "Register modal"
        <button on:click=run_req>"Request"</button>
    </Modal>
        }
}
