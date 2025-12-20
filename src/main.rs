use yew::prelude::*;
use yew_router::prelude::*;
use crate::{auth_context::AuthContext, router::{Route, switch}};

mod api;
mod auth_context;
mod router;
mod pages {
    pub mod login;
    pub mod dashboard;
}

#[function_component(App)]
fn app() -> Html {
    let auth_state = use_state(|| AuthContext { user: None });

    html! {
        <ContextProvider<UseStateHandle<AuthContext>> context={auth_state}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<UseStateHandle<AuthContext>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}