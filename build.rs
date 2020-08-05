// This simply pulls the msal-browser.js code into the root
use std::fs;
use std::io::prelude::*;

fn copy_msal_browser() {
    let source = fs::read_to_string("./node_modules/@azure/msal-browser/dist/index.es.js").unwrap();
    let mut output_path = std::env::current_dir().unwrap();
    output_path.push("msal-browser.js");

    let make_file = |source: String| {
        let mut output = fs::File::create(&output_path).unwrap();
        output.write_all(source.as_bytes()).unwrap()
    };

    match fs::read_to_string(&output_path) {
        Ok(existing) => {
            if existing != source {
                make_file(source);
            }
        }
        Err(_) => {
            make_file(source);
        }
    }
}

fn main() {
    copy_msal_browser();
}
