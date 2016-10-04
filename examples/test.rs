extern crate egg_mode;

mod common;
mod tests;

use std::io::Write;

fn main() {
    if std::fs::metadata("twitter_settings").is_ok() {
        print!("Existing twitter_settings file found. Would you like to create a new one? [y/N] ");
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        if buffer.starts_with("y") || buffer.starts_with("Y") {
            std::fs::rename("twitter_settings", "twitter_settings.bak").unwrap();
        }
    }

    let config = common::Config::load();

    tests::tweet::test_all(&config);

    println!("All done!");

    if std::fs::metadata("twitter_settings.bak").is_ok() {
        print!("Would you like to move your previous twitter_settings back? [y/N] ");
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        if buffer.starts_with("y") || buffer.starts_with("Y") {
            std::fs::rename("twitter_settings.bak", "twitter_setings").unwrap();
        }
    }
}
