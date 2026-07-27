use std::env;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

fn main() {
    const CUE_PACKAGE_DIR: &str = "./src/scanner";
    println!("cargo::rerun-if-changed={}", CUE_PACKAGE_DIR);

    let out_dir = env::var("OUT_DIR").unwrap();
    let output = match Command::new("cue")
        .args([
            "export",
            CUE_PACKAGE_DIR,
            "-f", // Force overwriting of potential existing file
            "--out",
            "json", // Output is a JSON file
            "-o",
            Path::new(&out_dir).join("re.json").to_str().unwrap(),
        ])
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => println!(
                    "cargo::error=Command cue not found. This is probably due to cue not being installed. Please follow https://cuelang.org/docs/introduction/installation/ to install cue."
                ),
                _ => println!("cargo::error=Unknown error: {}", error),
            }
            return;
        }
    };

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        println!(
            "cargo::error=Error while running cue command ({}). Error: {}",
            output.status,
            err_msg.trim().replace('\n', " | ")
        );
    }
}
