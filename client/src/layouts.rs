use crate::components::modal::*;
use crate::consts::*;
use crate::models::*;
use leptos::*;
use reqwasm::http::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SigninBody {
    username: String,
    pass: String,
}

#[component]
pub fn LoginModal(cx: Scope, is_open: RwSignal<bool>) -> impl IntoView {
    let context = use_context::<ContextStore>(cx).unwrap();

    let username = create_rw_signal(cx, String::new());
    let pw = create_rw_signal(cx, String::new());
    let error = create_rw_signal::<Option<String>>(cx, None);
    let sent = create_rw_signal(cx, 0);

    let send = move |_| {
        sent.update(|x| {
            *x += 1;
        })
    };

    let _ = create_resource(cx, sent, move |click_count| async move {
        if click_count == 0 {
            return ();
        }

        let url = format!("{}{}", API_URL, "/auth/signin");
        let body = SigninBody {
            username: username.get_untracked(),
            pass: pw.get_untracked(),
        };
        log!("{:?}", body);

        let parsed_body = serde_json::to_string(&body).unwrap();

        log!("{:?}", parsed_body);

        let resp = Request::post(url.as_str())
            .header("Content-Type", "application/json")
            .body(parsed_body)
            .send()
            .await
            .unwrap();

        let resp_data = resp.json::<User>().await.unwrap();

        context.user.set(Some(resp_data));
    });

    view! {
    cx,
            <Show when=move || context.user.get().is_none()
            fallback=|cx| view!{cx, <div>""</div> }
            >
        <Modal is_open=is_open>


            <label for="login-form-username">
            "Username"
            <input type="text" prop:value=username id="login-form-username" on:change=move |e| {
                let new_value = event_target_value(&e);

                username.set(new_value);
            } />
            </label>
            <label for="login-form-pw">
            "Password"
            <input type="password" prop:value=pw id="login-form-pw" on:change=move |e| {
                let new_value = event_target_value(&e);

                pw.set(new_value);
            } />
            </label>
            <button type="button" on:click=send>"Send"</button>
        </Modal>
        </Show>
        }
}
