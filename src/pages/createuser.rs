use std::fmt::format;

use gloo_net::http::Request;
use serde::Serialize;
use yew::prelude::*;
use yew_router::hooks::use_navigator;


use crate::router::Route;
use crate::{app_state::{AppData, AppState}, auth_context::AuthContext};

#[derive(Serialize)]
pub struct CreateUserForm {
    pub username: String,
    pub password: String,
    pub user_role: String,
}

#[function_component(CreateUserPage)]
pub fn create_user_page() -> Html {
    let auth_ctx = use_context::<UseStateHandle<AuthContext>>()
        .expect("AuthContext not found");
    let app_ctx = use_context::<UseStateHandle<AppState>>()
        .expect("AppState not found");

    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let user_role = use_state(|| String::new());
    let navigator = use_navigator().unwrap();
    let error_message = use_state(|| None::<String>);
    let show_pop_up = use_state(|| false);

    let on_submit = {
        let auth_ctx = auth_ctx.clone();
        let app_ctx = app_ctx.clone();
        let username = username.clone();
        let password = password.clone();
        let user_role = user_role.clone();
        let navigator = navigator.clone();
        let error_message = error_message.clone();
        let show_pop_up = show_pop_up.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let auth_ctx = auth_ctx.clone();
            let app_ctx = app_ctx.clone();
            let username = username.clone();
            let password = password.clone();
            let user_role = user_role.clone();
            let navigator = navigator.clone();
            let error_message = error_message.clone();
            let show_pop_up = show_pop_up.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let token = match &auth_ctx.token {
                    Some(t) => t.to_owned(),
                    None => return,
                };

                let auth_header = format!("Bearer {}", token);

                let payload = CreateUserForm {
                    username: (*username).clone(),
                    password: (*password).clone(),
                    user_role: (*user_role).clone(),
                };

                let resp = Request::post("http://127.0.0.1:8080/users")
                    .header(
                        "Authorization", 
                        &auth_header
                    )
                    .json(&payload)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(response) => {
                        if response.status() == 409 {
                            error_message.set(Some("Username already existed".to_string()));
                            show_pop_up.set(true);
                        } else if response.ok() {
                            show_pop_up.set(false);
                            error_message.set(None);

                            let data = Request::get("http://127.0.0.1:8080/app-data")
                                .header(
                                    "Authorization",
                                    format!("Bearer {}", token).as_ref()
                                )
                                .send()
                                .await
                                .unwrap()
                                .json::<AppData>()
                                .await
                                .unwrap();

                            app_ctx.set(AppState {
                                data: Some(data),
                                is_loading: false,
                            });

                            navigator.push(&Route::Dashboard);
                        }
                    }
                    Err(_) => {
                        error_message.set(Some("Network error".to_string()));
                        show_pop_up.set(true);
                    }
                }
            });
        })
    };

    html! {
        <div>
            <form
                onsubmit={on_submit}
            >
                <input
                    placeholder="Username"
                    oninput={Callback::from(move |e: InputEvent| {
                        username.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                    })}
                />
                <input
                    type="password"
                    placeholder="Passsword"
                    oninput={Callback::from(move |e: InputEvent| {
                        password.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                    })}
                />
                <input
                    placeholder="User Role"
                    oninput={Callback::from(move |e: InputEvent| {
                        user_role.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                    })}
                />
                <button>
                    {"Create User"}
                </button>
                {
                    if *show_pop_up {
                        html! {
                            <div>
                                <p>{error_message.clone().as_ref().unwrap()}</p>
                                <button
                                    onclick={{
                                        let show_pop_up = show_pop_up.clone();
                                        Callback::from(move |_| show_pop_up.set(false))
                                    }}
                                >
                                    {"Close"}
                                </button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </form>
        </div>
    }
}