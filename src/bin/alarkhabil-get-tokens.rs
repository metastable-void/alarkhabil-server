
use std::io::Write;

use alarkhabil_server::state::PrimarySecret;

fn main() {
    dotenvy::dotenv().expect(".env file not found");
    env_logger::init();

    let primary_secret = PrimarySecret::new_from_env();

    let invite_making_token = primary_secret.derive_secret("invite_making_token");
    let admin_token = primary_secret.derive_secret("admin_token");

    let result = serde_json::json!({
        "invite_making_token": hex::encode(invite_making_token),
        "admin_token": hex::encode(admin_token),
    });

    let mut stdout_lock = std::io::stdout().lock();
    serde_json::to_writer_pretty(&mut stdout_lock, &result).expect("Failed to write to stdout");

    // print newline
    writeln!(&mut stdout_lock).expect("Failed to write to stdout");
}
