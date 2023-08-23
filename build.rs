use std::process::Command;

fn main() {
    Command::new("npm").args(["install"]).output().unwrap();
    Command::new("postcss")
        .args(["-o", "./src/output.css", "./src/input.css"])
        .output()
        .unwrap();
}
