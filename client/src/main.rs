use leptos::*;
use leptos_router::*;

mod components;
use components::navigation::*;

mod pages;
use pages::main::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
    <Router>
    <Navigation />
    <main class="container mx-auto py-4">
        <Routes>
            <Route path="/" view=|cx| view! {cx, <MainPage />} />
            <Route path="/profile" view=|cx| view! {cx, <div>"Profile page"</div>} />
            <Route path="/account/:id" view=|cx| view! {cx, <div>"Account"</div>} />
        </Routes>
    </main>
    </Router>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}
