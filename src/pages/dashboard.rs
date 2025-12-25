use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::{auth_context::AuthContext, router::Route::{self}};

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    let auth_ctx = use_context::<UseStateHandle<AuthContext>>()
        .expect("AuthContext not found");

    html!{
        <div>
            <h1>{"Dashboard"}</h1>
            if let Some(user) = (*auth_ctx).user.clone() {
                <p>{format!("Welcome, {}", user.username)}</p>
            }
            <LogoutButton />
        </div>
    }
}

#[component]
pub fn LogoutButton() -> Html {
    let auth_ctx = use_context::<UseStateHandle<AuthContext>>()
        .expect("AuthContext not found");
    let navigator = use_navigator().unwrap();

    let on_click = {
        let navigator = navigator.clone();
        let auth_ctx = auth_ctx.clone();

        Callback::from(move |_| {
            LocalStorage::delete("token");
            auth_ctx.set(AuthContext {
                user: None,
                token: None
        });
        navigator.push(&Route::Login);
        })

    };

    html! {
        <button onclick={on_click}>
            {"Logout"}
        </button>
    }
}