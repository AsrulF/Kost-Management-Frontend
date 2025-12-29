use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub struct UserDto {
    pub username: String,
    pub user_id: u64,
    pub user_role: String,
}

#[derive(Deserialize, PartialEq)]
pub struct KostDto {
    pub kost_id: u64,
    pub kost_rooms: u8,
}


#[derive(Deserialize, PartialEq)]
pub struct AppData {
    pub users: Vec<UserDto>,
    pub kosts: Vec<KostDto>,
}

#[derive(PartialEq)]
pub struct AppState {
    pub data: Option<AppData>,
    pub is_loading: bool,
}