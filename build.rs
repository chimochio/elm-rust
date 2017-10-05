use std::process::Command;
use std::path::Path;
use std::fs::copy;

fn main() {
    println!("cargo:rerun-if-changed=frontend");

    let input_path = Path::new("App.elm");
    let dest_path = Path::new("../build/js/index.js");
    let out = Command::new("elm-make")
        .current_dir(Path::new("frontend"))
        .arg(input_path)
        .arg("--output")
        .arg(dest_path)
        .arg("--yes")
        .output().unwrap();

    copy("frontend/index.html", "build/index.html").expect("failed to copy index.html");

    if !out.status.success() {
        println!("{}", String::from_utf8_lossy(&out.stderr));
        std::process::exit(1);
    }
}
