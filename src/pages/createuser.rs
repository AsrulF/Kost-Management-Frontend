use yew::prelude::*;

use crate::app_state::AppState;



#[function_component(CreateUserPage)]
pub fn create_user_page() -> Html {
    let app_ctx = use_context::<UseStateHandle<AppState>>()
        .expect("AppState not found");

    let form = [
        "Username",
        "Password",
        "User Role",
    ];

    html! {
        <div>
            {
                for form.iter().map(|form_name| html! {
                    <>
                        <h2>{*form_name}</h2>
                        <input
                            placeholder={(*form_name).to_lowercase()}
                        />
                    </>
                })
            }
        </div>
    }
}