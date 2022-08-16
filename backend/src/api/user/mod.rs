pub mod generate_user;
pub mod get_user;

use serde::Serialize;
#[derive(Serialize)]
struct UserId {
    user_id: String,
}

