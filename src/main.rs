#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;
use std::collections::HashMap;
use eframe::{epi, egui};
use eframe::egui::Vec2;

mod map_downloader;
mod fmt;
mod lib;

#[derive(Default)]
struct Gui {
    dl_text: String,
    selected: String,
    success: u8,
}

impl epi::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Title
            ui.heading("Brawlhalla Map Theme Manager");
            ui.add_space(10_f32);

            // Download maps
            ui.horizontal(|ui| {
                if ui.button("Download Maps").clicked() {
                    self.dl_text = String::from("Downloading...");
                    let map_dir: PathBuf = map_downloader::maps_dir();
                    let manifest: HashMap<String, String> = map_downloader::get_manifest();
                    download_maps(&manifest, &map_dir);
                    self.dl_text = String::from("Downloaded.");
                };
                ui.label(&self.dl_text);
            });
            ui.add_space(30_f32);

            // Select map
            ui.radio_value(&mut self.selected, "dark".to_owned(), "dark");
            ui.radio_value(&mut self.selected, "blur".to_owned(), "blur");
            ui.radio_value(&mut self.selected, "dark blur".to_owned(), "dark blur");
            ui.radio_value(&mut self.selected, "default".to_owned(), "default (reset)");
            ui.add_space(10_f32);

            // Apply button
            if ui.button("Apply").clicked() {
                let manifest: HashMap<String, String> = map_downloader::get_manifest();
                let map_dir: PathBuf = map_downloader::maps_dir();
                let selected = &self.selected.clone();

                let tmp: Vec<&str> = manifest.get(selected).unwrap().split(".").collect();
                let folder_name = tmp[0];
                std::mem::drop(tmp);
                let chosen_dir = std::fs::read_dir(&map_dir.join(folder_name)).unwrap_or_else(|_| {
                    println!("Error in opening {}, do you have the maps downloaded?", selected);
                    std::process::exit(1);
                });

                // Get game path
                let game_path: PathBuf = lib::get_game_path();
                let image_path: PathBuf = game_path.join("images");
                let map_art_path: PathBuf = game_path.join("mapArt");
                for f in chosen_dir {
                    let folder = f.unwrap();
                    if folder.file_type().unwrap().is_dir() {
                        if folder.file_name() == "images" {
                            lib::copy_dir(&folder.path(), &image_path).unwrap();
                        } else if folder.file_name() == "mapArt" {
                            lib::copy_dir(&folder.path(), &map_art_path).unwrap();
                        }
                    }
                }
                self.success = 1_u8;
            }

            // Success label
            if self.success == 0_u8 {
                ui.label("Inactive");
            } else {
                ui.label("Success!");
            };
        });
    }

    fn name(&self) -> &str {
        "Brawlhalla Map Theme Manager"
    }
}

fn main() {
    let app = Gui {
        dl_text: String::from("Inactive"),
        selected: String::new(),
        success: 0_u8,
    };
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new( 280_f32, 225_f32 )),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
}

fn download_maps(manifest: &HashMap<String, String>, map_dir: &PathBuf) {
    println!("Downloading maps...");
    let base: String = String::from("https://bmaps.raforaweso.me/brawlhalla-maps/");
    let map_dir = map_dir.clone();

    for filename in manifest.values() {
        let mut full_url = base.clone();
        full_url.push_str(filename.as_str());
        let final_name = map_dir.clone().join(filename);
        println!("{} {:?}", &full_url, &final_name);
        let zipped_map: Vec<u8> = map_downloader::download_file(&full_url, final_name.to_str().unwrap());
        let tmp: Vec<&str> = filename.split(".").collect();  // .collect() requires a type annotation
        let folder_name: &str = tmp[0];  // so we split this into two variables
        std::mem::drop(tmp);
        let _ = map_downloader::unzip_map(zipped_map, &map_dir.join(folder_name));
        if std::fs::remove_file(&map_dir.join(final_name)).is_ok() {
            println!("Unzipped {}", filename);
        } else {
            println!("Failed in deleting extracted zip file!");
            std::process::exit(1);
        }
    }
}

// fn key_from_value<T: PartialEq>(map: HashMap<T, T>, va: T) -> Option<T> {
//     for (k, v) in map {
//         if v == va {
//             return Some(k);
//         }
//     };
//     None
// }

// Older CLI version code
// fn main() {
//     let _platform: &str = std::env::consts::OS;
//     let map_dir: PathBuf = map_downloader::maps_dir();
//     let manifest: HashMap<String, String> = map_downloader::get_manifest();
//     println!("{:?}", manifest);
//
//     // Download maps prompt
//     {  // Done like this so it's collapsible in IDEs
//         println!();
//         println!("Would you like to download available maps? (do this for the first time) [Y/n] ");
//         let mut input: String = String::new();
//         while input.is_empty() {
//             let mut temp: String = String::new();
//             let _ = std::io::stdin().read_line(&mut temp);
//             match temp.to_lowercase().trim() {
//                 "y" => {
//                     input = String::from("y");
//                     download_maps(&manifest, &map_dir);
//                 },
//                 "n" => {
//                     input = String::from("n");
//                 },
//                 _ => {
//                     println!("Invalid input, please try again.\n");
//                 }
//             }
//         }
//         std::mem::drop(input);
//     }
//
//     // Select which map to use
//     let options: Vec<&String> = manifest.keys().collect();
//
//     for (i, v) in options.iter().enumerate() {
//         println!("{}. {}", i + 1, v);
//     }
//
//     let mut selected: &str = "";
//     while selected.is_empty() {
//         let mut choice: String = String::new();
//         let _ = std::io::stdin().read_line(&mut choice);
//         let choice_num = choice.trim().parse::<i32>();
//         if let Ok(..) = choice_num {
//             let unwrapped: i32 = choice_num.unwrap();
//             if options.len() >= unwrapped as usize && unwrapped > 0 {
//                 selected = options[(unwrapped - 1) as usize];
//             } else {
//                 println!("Invalid choice!");
//             }
//         } else {
//             println!("Input is not a number!");
//         }
//     }
//     println!();
//     println!("Opening {}", selected);
//     let tmp: Vec<&str> = manifest.get(selected).unwrap().split(".").collect();
//     let folder_name = tmp[0];
//     std::mem::drop(tmp);
//     let chosen_dir = std::fs::read_dir(&map_dir.join(folder_name)).unwrap_or_else(|_| {
//         println!("Error in opening {}, do you have the maps downloaded?", selected);
//         std::process::exit(1);
//     });
//
//     // Get game path
//     let game_path: PathBuf = lib::get_game_path();
//     let image_path: PathBuf = game_path.join("images");
//     let map_art_path: PathBuf = game_path.join("mapArt");
//     for f in chosen_dir {
//         let folder = f.unwrap();
//         if folder.file_type().unwrap().is_dir() {
//             if folder.file_name() == "images" {
//                 lib::copy_dir(&folder.path(), &image_path).unwrap();
//             } else if folder.file_name() == "mapArt" {
//                 lib::copy_dir(&folder.path(), &map_art_path).unwrap();
//             }
//         }
//     }
//
//     println!("Successfully changed map theme to {}.", selected);
// }