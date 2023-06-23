use leptos::*;
use reqwasm::http::*;

async fn run_req(_: ()) -> Result<String, String> {
    let resp = Request::get("https://swapi.dev/api/people/")
        .send()
        .await
        .unwrap();

    resp.text().await.map_err(|_| "Coulnt fetch".to_string())
}

#[component]
pub fn MainPage(cx: Scope) -> impl IntoView {
    let async_data = create_resource(cx, || (), run_req);

    view! {cx,
    <div class="container mx-auto">
    {move || match async_data.read(cx) {
                None => view!{cx, <p>"Not logged in"</p>},
                Some(d) => view!{cx,
                    <p>{format!("{:?}", d)}</p>
                }
            }}
    <p>"This is a main page"</p>
    </div>
        }
}
