// This simply pulls the msal-browser.js code into the root
use fs::File;
use std::fs;
use std::io::prelude::*;

fn copy_msal_browser() {
    let current_dir = std::env::current_dir().unwrap();
    let mut output_path = current_dir.clone();
    output_path.push("msal-browser.js");
    let base_path = "./node_modules/@azure/msal-browser/dist/index.es.js";

    let msal_js = if let Ok(_) = File::open(&base_path) {
        fs::read_to_string(&base_path).unwrap()
    } else {
        let mut in_target = String::from("../../../");
        in_target.push_str(&base_path);
        fs::read_to_string(in_target).unwrap()
    };

    let make_file = |source: String| {
        let mut output = fs::File::create(&output_path).unwrap();
        output.write_all(source.as_bytes()).unwrap()
    };

    match fs::read_to_string(&output_path) {
        Ok(existing) => {
            if existing != msal_js {
                make_file(msal_js);
            }
        }
        Err(_) => {
            make_file(msal_js);
        }
    }
}

fn main() {
    copy_msal_browser();
}
