use std::process::Command;

fn new_npm() -> Command {
    const NPM: &str = "npm";
    if cfg!(windows) {
        let mut cmd = Command::new("cmd");
        cmd.args(["/c", NPM]);
        cmd
    } else {
        Command::new(NPM)
    }
}

fn main() {
    println!("cargo:rerun-if-changed=package.json");

    let js = "js/msal-browser.js";
    let js_map = format!("{}.map", js);
    std::fs::remove_file(js).unwrap_or(());
    std::fs::remove_file(js_map).unwrap_or(());

    new_npm().arg("install").spawn().unwrap().wait().unwrap();
    let output = new_npm().args(["run", "build"]).output().unwrap();

    // https://nodejs.org/api/process.html#process_exit_codes
    if output.status.code() != Some(0) {
        panic!(
            "Rollup failed with:\n{}",
            String::from_utf8(output.stderr).unwrap()
        )
    }
}
