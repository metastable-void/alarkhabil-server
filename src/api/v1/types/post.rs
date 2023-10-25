
use serde::{Serialize, Deserialize};

use crate::api::v1::types::{
    ChannelSummary,
    AuthorSummary,
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionInfo {
    uuid: String,
    author: AuthorSummary,
    created_date: u64,
    title: String,
    revision_text: String,
}

impl RevisionInfo {
    pub fn new(uuid: &str, author: &AuthorSummary, created_date: u64, title: &str, revision_text: &str) -> RevisionInfo {
        RevisionInfo {
            uuid: uuid.to_string(),
            author: author.clone(),
            created_date,
            title: title.to_string(),
            revision_text: revision_text.to_string(),
        }
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn author(&self) -> &AuthorSummary {
        &self.author
    }

    pub fn created_date(&self) -> u64 {
        self.created_date
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn revision_text(&self) -> &str {
        &self.revision_text
    }
}


/// PostInfo is a struct that contains detailed information about a post.
/// It is for example returned by `/api/v1/post/info`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostInfo {
    post_uuid: String,
    channel: ChannelSummary,
    tags: Vec<String>,
    revision_uuid: String,
    revision_date: u64,
    title: String,
    revision_text: String,
    author: AuthorSummary,
}

impl PostInfo {
    pub fn new(uuid: &str, channel: &ChannelSummary, tags: Vec<String>, revision: &RevisionInfo, author: &AuthorSummary) -> PostInfo {
        PostInfo {
            post_uuid: uuid.to_string(),
            channel: channel.clone(),
            tags,
            revision_uuid: revision.uuid().to_string(),
            revision_date: revision.created_date(),
            title: revision.title().to_string(),
            revision_text: revision.revision_text().to_string(),
            author: author.clone(),
        }
    }

    pub fn post_uuid(&self) -> &str {
        &self.post_uuid
    }

    pub fn channel(&self) -> &ChannelSummary {
        &self.channel
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn revision_uuid(&self) -> &str {
        &self.revision_uuid
    }

    pub fn revision_date(&self) -> u64 {
        self.revision_date
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn revision_text(&self) -> &str {
        &self.revision_text
    }

    pub fn author(&self) -> &AuthorSummary {
        &self.author
    }
}
