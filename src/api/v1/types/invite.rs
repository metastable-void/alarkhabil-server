
use serde::{Serialize, Deserialize};
use monostate::MustBe;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invite {
    command: MustBe!("registration_invite"),
    uuid: String,
}

impl Invite {
    pub fn new() -> Invite {
        let uuid = uuid::Uuid::new_v4().to_string();
        Invite {
            command: MustBe!("registration_invite"),
            uuid,
        }
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }
}
