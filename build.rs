use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=package.json");
    
    let js = "js/msal-browser.js";
    let js_map = format!("{}.map", js);
    std::fs::remove_file(js).unwrap_or(());
    std::fs::remove_file(js_map).unwrap_or(());

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "npm install"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("npm install")
            .output()
            .expect("failed to execute process")
    };

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "npm run build"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("npm run build")
            .output()
            .expect("failed to execute process")
    };

    // https://nodejs.org/api/process.html#process_exit_codes
    if output.status.code() != Some(0) {
        panic!(
            "Rollup failed with:\n{}",
            String::from_utf8(output.stderr).unwrap()
        )
    }
}
