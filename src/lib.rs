
pub mod base64;
pub mod sys_time;
pub mod state;
pub mod api;
pub mod crypto;

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewInviteResult {
    #[serde(with="crate::base64")]
    pub invite: Vec<u8>,
}
