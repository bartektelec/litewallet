use leptos::*;
use leptos_router::*;

mod components;

use components::input::TodoInput;
use components::todo::Todo;

#[derive(Clone)]
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub checked: bool,
    pub active: bool,
}

#[derive(Clone)]
struct Store {
    pub remove_todo: WriteSignal<Option<usize>>,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal::<Vec<TodoItem>>(cx, vec![]);
    let (remove_id, set_remove_id) = create_signal::<Option<usize>>(cx, None);

    let add_todo = move |text: String| {
        let new_id = if let Some(last_todo) = todos.get().last() {
            last_todo.id + 1
        } else {
            0
        };

        set_todos.update(|collection| {
            collection.push(TodoItem {
                id: new_id,
                title: text,
                active: true,
                checked: false,
            })
        })
    };

    create_effect(cx, move |_| {
        if let Some(id_to_remove) = remove_id() {
            set_todos.update(move |collection| {
                if let Some(found) = collection.iter().position(|t| t.id == id_to_remove) {
                    collection.remove(found);
                }
            });
        };
    });

    provide_context(
        cx,
        Store {
            remove_todo: set_remove_id,
        },
    );
    view! { cx,
    <Router>
        <a href="/">"Main page"</a>
        <a href="/about">"About page"</a>
        <TodoInput on_add=add_todo />
        <For each=todos
        key=|t| {t.id}
        view=|cx, todo| view!{cx, <Todo todo />} />
        <Routes>

        <Route path="/" view=|cx| view! {cx, <div>"Main"</div>} />
        <Route path="/about" view=|cx| view! {cx, <div>"About"</div>} />
        </Routes>
    </Router>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}
