
pub mod types;

// API modules
mod invite_new;

mod account_new;
mod account_change_credentials;
mod account_delete;

mod admin_meta_update;
mod admin_meta_delete;
mod admin_author_delete;
mod admin_channel_delete;
mod admin_post_delete;

mod self_update;
mod channel_new;
mod channel_update;
mod channel_delete;

mod meta_info;


// API handlers
pub use invite_new::api_invite_new;

pub use account_new::api_account_new;
pub use account_change_credentials::api_account_change_credentials;
pub use account_delete::api_account_delete;

pub use admin_meta_update::api_admin_meta_update;
pub use admin_meta_delete::api_admin_meta_delete;
pub use admin_author_delete::api_admin_author_delete;
pub use admin_channel_delete::api_admin_channel_delete;
pub use admin_post_delete::api_admin_post_delete;

pub use self_update::api_self_update;
pub use channel_new::api_channel_new;
pub use channel_update::api_channel_update;
pub use channel_delete::api_channel_delete;

pub use meta_info::api_meta_info;
