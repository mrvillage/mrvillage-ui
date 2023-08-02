use std::process::Command;

fn main() {
    Command::new("postcss")
        .args(["-o", "./src/output.css", "./src/input.css"])
        .output()
        .unwrap();
}
