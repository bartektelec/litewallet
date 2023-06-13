use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! {
            cx,
    <footer class="py-8 bg-gray-100 items-center flex justify-center">
         <p class="container text-sm font-semibold text-center mx-auto">"2023 Litewallet - Fake product"</p>
    </footer>
        }
}
