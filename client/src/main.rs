mod components;
mod consts;
mod layouts;
mod models;
mod pages;

use leptos::*;
use leptos_router::*;

use components::footer::*;
use components::navigation::*;

use pages::main::*;

use models::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let user = create_rw_signal::<Option<User>>(cx, None);
    let x = create_rw_signal(cx, 0);

    let y = x.get();
    provide_context(cx, ContextStore { user });

    view! { cx,
    <Router>
    <Navigation/>
    <main class="container flex flex-col flex-1 mx-auto py-4">
        <Routes>
            <Route path="/" view=|cx| view! {cx, <MainPage />} />
            <Route path="/profile" view=|cx| view! {cx, <div>"Profile page"</div>} />
            <Route path="/account/:id" view=|cx| view! {cx, <div>"Account"</div>} />
        </Routes>
    </main>
    <Footer/>
    </Router>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}
