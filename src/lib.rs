pub mod cron_manager;

use nix::libc::getuid;
use users::get_user_by_uid;
pub fn current_user() -> Option<String> {
    let uid = unsafe { getuid() };
    if let Some(user) = get_user_by_uid(uid) {
        Some(user.name().to_str().unwrap().to_string())
    } else {
        None
    }
}

fn main() {
    match current_user() {
        Some(username) => println!("Current username: {}", username),
        None => println!("Failed to get the current username"),
    }
}
