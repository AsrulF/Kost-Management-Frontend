use yew::prelude::*;
use crate::auth_context::AuthContext;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    let auth_ctx = use_context::<UseStateHandle<AuthContext>>()
        .expect("AuthContext not found");

    html!{
        <div>
            <h1>{"Dashboard"}</h1>
            if let Some(user) = &auth_ctx.user {
                <p>{format!("Welcome, {}", user.username)}</p>
            }
        </div>
    }
}