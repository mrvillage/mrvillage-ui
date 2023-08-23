use std::process::Command;

fn main() {
    println!("TEST");
    println!("{}", std::str::from_utf8(&Command::new("npm").args(["install"]).output().unwrap().stdout).unwrap());
    println!("{}", std::str::from_utf8(&Command::new("postcss")
        .args(["-o", "./css/output.css", "./css/input.css"])
        .output()
        .unwrap().stdout).unwrap());
}
