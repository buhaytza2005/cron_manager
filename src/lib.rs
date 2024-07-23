use nix::libc::getuid;
use users::get_user_by_uid;
fn current_user() -> Option<String> {
    let uid = getuid().as_raw();
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
