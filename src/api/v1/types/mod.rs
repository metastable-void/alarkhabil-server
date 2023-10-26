
mod invite;
mod author;
mod channel;
mod post;


pub use invite::Invite;

pub use author::AuthorInfo;
pub use author::AuthorSummary;

pub use channel::validate_channel_handle;
pub use channel::ChannelInfo;
pub use channel::ChannelSummary;

pub use post::RevisionInfo;
pub use post::PostInfo;


use regex::Regex;

pub fn validate_language_code(lang: &str) -> Result<(), anyhow::Error> {
    let re = Regex::new(r"^[a-z]{2, 3}(-[A-Z]{2})?$").unwrap();
    if !re.is_match(lang) {
        return Err(anyhow::anyhow!("Invalid language code"));
    }
    Ok(())
}

pub fn is_valid_dns_token(token: &str) -> bool {
    let re = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();
    token.len() <= 64 && re.is_match(token)
}

/// UUIDs are expected to be in lowercase.
pub fn validate_v4_uuid(uuid: &str) -> Result<(), anyhow::Error> {
    let re = Regex::new(r"[A-F]").unwrap();
    if re.is_match(uuid) {
        return Err(anyhow::anyhow!("UUIDs must be lowercase"));
    }
    let uuid = uuid.parse::<uuid::Uuid>()?;
    if uuid.get_version() != Some(uuid::Version::Random) {
        return Err(anyhow::anyhow!("Invalid uuid version"));
    }
    Ok(())
}
