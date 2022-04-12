use std::path::PathBuf;
mod map_downloader;
mod fmt;

fn main() {
    let js: String = String::from("{\n  \"e\": \"e\"\n}");
    let parsed = fmt::json_parse(js);
    println!("{:?}", parsed);
}
