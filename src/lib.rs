use std::path::PathBuf;

pub fn get_game_path() -> PathBuf {
    let platform: &str = std::env::consts::OS;
    let game_path: PathBuf;
    let homedir: PathBuf = home::home_dir().unwrap_or_else(|| {
        println!("Failed to get home directory!");
        std::process::exit(1);
    });
    if platform == "windows" {
        game_path = PathBuf::from("");
    } else if platform == "macos" {
        game_path = homedir.join("Library/Application Support/Steam/steamapps/common/Brawlhalla/Brawlhalla.app/Contents/Resources");
    } else if platform == "linux" {
        game_path = PathBuf::from("/usr/bin/Brawlhalla");
    } else {
        game_path = PathBuf::new();  // Silence rust warnings
        println!("Unsupported platform!");
        std::process::exit(1);
    }
    game_path
}