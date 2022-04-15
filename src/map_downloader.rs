use std::path::PathBuf;
use std::collections::HashMap;
use std::env;
use crate::fmt;
use std::io::Cursor;

pub fn maps_dir() -> PathBuf {
    let mut base_path = PathBuf::new();
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

    if !exists {
        dbg!(&map_path);
        let res = std::fs::create_dir(&map_path);
        if res.is_err() {
            // dbg!(res.err());
            eprintln!("Could not create maps directory.");
            std::process::exit(1);
        }
    }
    map_path
}

// pub fn download_file(link: &str, outfile: &str) {
//     let mut outfile = std::fs::File::create(outfile).unwrap_or_else(|_| {
//         eprintln!("Error occurred in creating download file");
//         std::process::exit(1);
//     });
//     let response = request(link);
//     std::io::copy(&mut response.as_bytes(), &mut outfile).unwrap_or_else(|_| {
//         eprintln!("Error copying file");
//         std::process::exit(1);
//     });
// }

pub fn download_file(link: &str, outfile: &str) -> Vec<u8> {
    let outfile_string = String::from(outfile);
    let mut outfile = std::fs::File::create(outfile).unwrap_or_else(|_| {
        eprintln!("Error occurred in creating download file");
        std::process::exit(1);
    });
    reqwest::blocking::get(link).unwrap().copy_to(&mut outfile).unwrap_or_else(|_| {
        println!("Failed to copy downloaded file.");
        std::process::exit(1);
    });

    std::fs::read(PathBuf::from(outfile_string)).unwrap_or_else(|_| {
        println!("Error in reading downloaded file!");
        std::process::exit(1);
    })
}

pub fn get_manifest() -> HashMap<String, String> {
    let retrieved = request("https://bmaps.raforaweso.me/brawlhalla-maps/manifest.json");
    let map: HashMap<String, String> = fmt::json_parse(retrieved);
    map
}

pub fn unzip_map(map: Vec<u8>, target: &PathBuf) -> Result<(), ()> {
    let res = zip_extract::extract(Cursor::new(map), target, false);
    match res {
        Ok(_) => {
            Ok(())
        },
        Err(_) => {
            Err(())
        }
    }
}

#[tokio::main]
async fn request(link: &str) -> String {
    let b1 = reqwest::get(link);
    let b2 = b1.await.unwrap();
    let body = b2.text().await.unwrap();
    body
}