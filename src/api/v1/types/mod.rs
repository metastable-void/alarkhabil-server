
mod invite;
mod author;
mod channel;


pub use invite::Invite;

pub use author::AuthorInfo;
pub use author::AuthorSummary;

pub use channel::validate_channel_handle;
pub use channel::ChannelInfo;
pub use channel::ChannelSummary;


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
