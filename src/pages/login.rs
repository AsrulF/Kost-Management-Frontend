use yew::prelude::*;
use yew_router::prelude::*;
use crate::{api, auth_context::AuthContext, router::Route};
use gloo_storage::{LocalStorage, Storage};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| None::<String>);
    let navigator: Navigator = use_navigator().unwrap();
    let auth_ctx = use_context::<UseStateHandle<AuthContext>>()
        .expect("AuthContext not found");

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        let auth_ctx = auth_ctx.clone();

        Callback::from(move |_| {
            let username = (*username).clone();
            let password = (*password).clone();
            let error = error.clone();
            let navigator = navigator.clone();
            let auth_ctx = auth_ctx.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match api::login(username, password).await {
                    Ok(auth) => {
                        LocalStorage::set("token", auth.token.clone()).unwrap();
                        auth_ctx.set(AuthContext {
                            user: Some(auth.user),
                            token: Some(auth.token),
                        });
                        navigator.push(&Route::Dashboard);
                    },
                    Err(_) => error.set(Some("Login Failed".into())),
                }
            });
        })
    };

    html! {
        <div>
            <h2>{"Login"}</h2>
            <input
                placeholder="username"
                oninput={Callback::from(move |e: InputEvent| {
                    username.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                })}
            />
            <input
                type="password"
                placeholder="password"
                oninput={Callback::from(move |e: InputEvent| {
                    password.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                })}
            />
            <button
                onclick={on_submit}
            >
                {"Login"}
            </button>
            {
                if let Some(err) = &*error {
                    html! {<p style="color:red">{err}</p>}
                } else {
                    html! {}
                }
            }
        </div>
    }
}
