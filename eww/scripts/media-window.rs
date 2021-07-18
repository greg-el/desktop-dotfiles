use std::process::Command;

fn main() {
    let bar = Command::new("eww")
        .arg("windows")
        .output()
        .expect("Failed to get bar windows");

    let windows_str = String::from_utf8_lossy(&bar.stdout);
    let windows: Vec<&str> = windows_str.split("\n").collect();
    if windows.contains(&"*media") {
        Command::new("eww")
            .arg("close")
            .arg("media")
            .output()
            .expect("Failed to close Media");
    } else {
        Command::new("eww")
            .arg("open")
            .arg("media")
            .output()
            .expect("Failed to open media");
    }
}