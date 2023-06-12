use leptos::*;

#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    view! {cx,
    <nav class="bg-gray-800 py-3 text-white">
        <div class="container flex justify-between items-center mx-auto">

        <a href="/" class="font-semibold text-lg">"Litewallet"</a>

        <div class="flex gap-2 items-center">
        <button class="btn btn-secondary">"Sign up"</button>
        <button class="btn btn-primary">"Log in"</button>
        </div>
        </div>
    </nav>
        }
}
