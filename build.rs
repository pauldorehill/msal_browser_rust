use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=package.json");

    let js = "js/msal-browser.js";
    let js_map = format!("{}.map", js);
    std::fs::remove_file(js).unwrap_or(());
    std::fs::remove_file(js_map).unwrap_or(());

    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "npm install"]).output()
    } else {
        Command::new("sh").args(["-c", "npm install"]).output()
    }
    .unwrap();

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "npm run build"]).output()
    } else {
        Command::new("sh").args(["-c", "npm run build"]).output()
    }
    .unwrap();

    // https://nodejs.org/api/process.html#process_exit_codes
    if output.status.code() != Some(0) {
        panic!(
            "Rollup failed with:\n{}",
            String::from_utf8(output.stderr).unwrap()
        )
    }
}
