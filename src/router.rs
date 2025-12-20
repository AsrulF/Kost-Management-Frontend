use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{login::LoginPage, dashboard::DashboardPage};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/dashboard")]
    Dashboard
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! {<LoginPage />},
        Route::Dashboard => html! {<DashboardPage />}
    }
}