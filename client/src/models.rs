use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Clone)]
pub struct ContextStore {
    pub user: RwSignal<Option<User>>,
}
