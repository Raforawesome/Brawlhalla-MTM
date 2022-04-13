use std::path::PathBuf;
use std::collections::HashMap;
use rayon::prelude::*;
mod map_downloader;
mod fmt;

fn main() {
    let platform: &str = std::env::consts::OS;
    let map_dir: PathBuf = map_downloader::maps_dir();
    let manifest: HashMap<String, String> = map_downloader::get_manifest();
    println!("{:?}", manifest);

    // Download maps prompt
    {
        println!();
        println!("Would you like to download available maps? (do this for the first time) [Y/n] ");
        let mut input: String = String::new();
        while input.is_empty() {
            let mut temp: String = String::new();
            let _ = std::io::stdin().read_line(&mut temp);
            match temp.to_lowercase().trim() {
                "y" => {
                    input = String::from("y");
                    download_maps(&manifest, &map_dir);
                },
                "n" => {
                    input = String::from("n");
                },
                _ => {
                    println!("Invalid input, please try again.\n");
                }
            }
        }
        std::mem::drop(input);
    }

    // Get game path
    let mut game_path: PathBuf = PathBuf::new();
    let homedir: PathBuf = std::env::current_dir().unwrap_or_else(|_| {
        println!("Failed to get home directory!");
        PathBuf::new()
    });
    if platform == "windows" {
        game_path = PathBuf::from("");
    } else if platform == "macos" {
        game_path = homedir.clone().join("Library/Application Support/Steam/steamapps/common/Brawlhalla/Brawlhalla.app/Contents/Resources");
    } else if platform == "linux" {
        game_path = PathBuf::from("/usr/bin/Brawlhalla");
    } else {
        println!("Unsupported platform!");
        std::process::exit(1);
    }


    // let input: String = String::new();
    // while &input == "" {
    //     let mut temp: String = String::new();
    //     let _ = std::io::stdin().read_line(&mut temp);
    //     match temp.as_str() {
    //         "1" => {},
    //         "2" => {},
    //         "3" => {},
    //         "4" => {},
    //         _ => {
    //             println!("Invalid input, please try again.\n");
    //         }
    //     }
    // }
}

fn download_maps(manifest: &HashMap<String, String>, map_dir: &PathBuf) {
    println!("Downloading maps...");
    let base: String = String::from("https://raforaweso.me/brawlhalla-maps/");
    let map_dir = map_dir.clone();

    for filename in manifest.values() {
        let mut full_url = base.clone();
        full_url.push_str(filename.as_str());
        let final_name = map_dir.clone().join(filename);
        println!("{} {:?}", &full_url, &final_name);
        map_downloader::download_file(&full_url, final_name.to_str().unwrap());
    }
}

fn key_from_value<T: PartialEq>(map: HashMap<T, T>, va: T) -> Option<T> {
    for (k, v) in map {
        if v == va {
            return Some(k);
        }
    };
    None
}
