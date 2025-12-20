use yew::prelude::*;
use crate::api::User;

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub user: Option<User>,
}

impl AuthContext {
    pub fn is_logged_in(&self) -> bool {
        self.user.is_some()
    }
}