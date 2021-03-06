use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildMemberRemove {
    pub guild_id: String,
    pub user: User,
}
