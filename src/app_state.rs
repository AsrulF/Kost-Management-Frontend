use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, PartialEq)]
pub struct UserDto {
    pub username: String,
    pub user_id: Uuid,
    pub user_role: String,
}

#[derive(Deserialize, PartialEq)]
pub struct KostDto {
    pub kost_id: Uuid,
    pub kost_rooms: u8,
}


#[derive(Deserialize, PartialEq)]
pub struct AppData {
    pub users: Vec<UserDto>,
    pub kosts: Vec<KostDto>,
}

#[derive(PartialEq,)]
pub struct AppState {
    pub data: Option<AppData>,
    pub is_loading: bool,
}