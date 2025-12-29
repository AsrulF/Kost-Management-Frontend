use yew::prelude::*;
use yew_router::prelude::*;
use crate::{app_state::AppState, auth_context::AuthContext, pages::{dashboard::DashboardPage, login::LoginPage}, router::Route};

mod api;
mod auth_context;
mod router;
mod app_state;
mod pages {
    pub mod login;
    pub mod dashboard;
}

#[function_component(App)]
fn app() -> Html {
    let auth_state = use_state(|| AuthContext { user: None , token: None });
    let app_state = use_state(|| AppState { data: None, is_loading: true });

    html! {
        <ContextProvider<UseStateHandle<AuthContext>> context={auth_state.clone()}>
            <ContextProvider<UseStateHandle<AppState>> context={app_state.clone()}>
                <BrowserRouter>
                    <Switch<Route> render={move |route| {
                        let auth = (*auth_state).clone();

                        match route {
                            Route::Login => html!{ <LoginPage /> },
                            Route::Dashboard => {
                                if auth.user.is_some() && auth.token.is_some() {
                                    html! { <DashboardPage />}
                                } else {
                                    html! { <LoginPage /> }
                                }
                            }
                        }

                    }} />
                </BrowserRouter>
            </ContextProvider<UseStateHandle<AppState>>>
        </ContextProvider<UseStateHandle<AuthContext>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}