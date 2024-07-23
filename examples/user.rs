use cron_manager::current_user;

fn main() {
    match current_user() {
        Some(username) => println!("Current username: {}", username),
        None => println!("Failed to get the current username"),
    }
}
