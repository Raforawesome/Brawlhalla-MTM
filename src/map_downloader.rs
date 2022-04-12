use std::path::PathBuf;
use std::collections::HashMap;
use std::env;

pub async fn maps_dir() -> PathBuf {
    let mut base_path;
    if let Ok(path) = env::current_exe() {
        base_path = path;
    } else {
        eprintln!("Could not get current directory.");
        std::process::exit(1);
    }
    base_path = base_path.parent().unwrap().to_path_buf();
    let map_path: PathBuf = base_path.join("maps");
    std::mem::drop(base_path);
    let exists = map_path.exists();

    if exists == false {
        dbg!(&map_path);
        let res = std::fs::create_dir(&map_path);
        if res.is_err() {
            // dbg!(res.err());
            eprintln!("Could not create maps directory.");
            std::process::exit(1);
        }
    }
    let _e = get_manifest().await;
    map_path
}

async fn get_manifest() -> HashMap<String, String> {
    let map: HashMap<String, String> = HashMap::new();
    let retrieved = request().await;
    json_parse(retrieved);
    map
}

async fn request() -> String {
    let b1 = reqwest::get("https://raforaweso.me/brawlhalla-maps/manifest.json");
    let b2 = b1.await.unwrap();
    let body = b2.text().await.unwrap();
    body
}