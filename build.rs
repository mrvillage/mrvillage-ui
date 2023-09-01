use std::process::Command;

fn main() {
    for entry in std::fs::read_dir("./src").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path = path.to_str().unwrap();
            if path.ends_with(".rs") {
                println!("cargo:rerun-if-changed={}", path);
            }
        }
    }
    println!(
        "{}",
        std::str::from_utf8(
            &Command::new("npm")
                .args(["install"])
                .output()
                .unwrap()
                .stdout
        )
        .unwrap()
    );
    println!(
        "{}",
        std::str::from_utf8(
            &Command::new("postcss")
                .args(["-o", "./css/output.css", "./css/input.css"])
                .output()
                .unwrap()
                .stdout
        )
        .unwrap()
    );
}
