use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::{app_state::{AppData, AppState}, auth_context::AuthContext, router::Route::{self}};

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    let auth_ctx = use_context::<UseStateHandle<AuthContext>>()
        .expect("AuthContext not found");

    let app_ctx = use_context::<UseStateHandle<AppState>>()
        .expect("AppState not found");

    let token_clone = auth_ctx.token.clone();

    use_effect_with(
        auth_ctx.user.clone(), 
        move |user| {
            if let Some(_) = user {
                let token = token_clone.clone();
                let app_state = app_ctx.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let token = match token {
                        Some(t) => t,
                        None => return,
                    };

                    app_state.set(AppState {
                        data: None,
                        is_loading: true,
                    });

                    let data = Request::get("http://127.0.0.1:8080/app-data")
                        .header(
                            "Authorization",
                            format!("Bearer {}", token).as_str()
                        )
                        .send()
                        .await
                        .unwrap()
                        .json::<AppData>()
                        .await
                        .unwrap();

                    app_state.set(AppState {
                        data: Some(data),
                        is_loading: false
                    });
                });
            }

            || ()
        }, 
    );

    html!{
        <div>
            <h1>{"Dashboard"}</h1>

            if let Some(user) = &auth_ctx.user.clone() {
                <p>{format!("Welcome, {}", user.username)}</p>
            } 

            <LogoutButton />
            <NavBar />
        </div>
    }
}

#[component]
pub fn LogoutButton() -> Html {
    let auth_ctx = use_context::<UseStateHandle<AuthContext>>()
        .expect("AuthContext not found");

    let on_click = {
        let auth_ctx = auth_ctx.clone();

        Callback::from(move |_| {
            LocalStorage::delete("token");
            auth_ctx.set(AuthContext {
                user: None,
                token: None
        });
        })

    };

    html! {
        <button onclick={on_click}>
            {"Logout"}
        </button>
    }
}

#[component]
pub fn NavBar() -> Html {
    let navigator = use_navigator().unwrap();


    let create_user_on_click = {
        let navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::CreateUser);
        })
    };

    html! {
        <div>
            <button
                onclick={create_user_on_click}
            >
                {"Create User"}
            </button>
            <button>
                {"Kost Details"}
            </button>
        </div>
    }
}