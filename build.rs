use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=package.json");
    
    let js = "js/msal-browser.js";
    let js_map = format!("{}.map", js);
    std::fs::remove_file(js).unwrap_or(());
    std::fs::remove_file(js_map).unwrap_or(());

    let output = Command::new("npm").args(["run", "build"]).output().unwrap();
    // https://nodejs.org/api/process.html#process_exit_codes
    if output.status.code() != Some(0) {
        panic!(
            "Rollup failed with:\n{}",
            String::from_utf8(output.stderr).unwrap()
        )
    }
}
