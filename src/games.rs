//! API Objects to be used with [HTML5 games](https://core.telegram.org/bots/api#games).

use crate::macros::{apistruct, apply};
use crate::types::{Animation, MessageEntity, PhotoSize, User};

#[apply(apistruct!)]
#[derive(Copy, Eq)]
pub struct CallbackGame {}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[apply(apistruct!)]
#[derive(Eq)]
pub struct GameHighScore {
    pub position: u32,
    pub user: User,
    pub score: i32,
}
