use leptos::html::Dialog;
use leptos::*;

#[component]
pub fn Modal(
    cx: Scope,
    #[prop(default = String::new())] title: String,
    is_open: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let dialog_ref = create_node_ref::<Dialog>(cx);

    create_effect(cx, move |_| {
        if let Some(dialog) = dialog_ref.get() {
            if is_open.get() == true {
                let _ = dialog.show_modal();
            } else {
                dialog.close();
            }
        }
    });

    let handle_close = move || {
        is_open.set(false);
    };

    view! {
        cx,
        <dialog
            class="min-w-[180px] p-0 rounded flex-col"
            class:flex=is_open
            node_ref=dialog_ref on:close=move |_| {
            handle_close();
        }>
            <header class="p-3 border-b gap-4 border-gray-200 flex justify-between items-center">
                <div>{title}</div>
                <button on:click=move |_| {
                    handle_close();
                }>"X"</button>
            </header>
                <main class="flex p-4 flex-col">
        {children(cx)}
            </main>
        </dialog>
    }
}
