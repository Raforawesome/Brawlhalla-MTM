use std::path::PathBuf;

pub fn get_game_path() -> PathBuf {
    let platform: &str = std::env::consts::OS;
    let game_path: PathBuf;
    let homedir: PathBuf = home::home_dir().unwrap_or_else(|| {
        println!("Failed to get home directory!");
        std::process::exit(1);
    });
    if platform == "windows" {
        game_path = PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Brawlhalla");
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

pub fn copy_dir(from: &PathBuf, to: &PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(&to)?;
    for file in std::fs::read_dir(&from)? {
        let file = file?;
        let t = file.file_type()?;
        if t.is_dir() {
            copy_dir(&file.path(), &to.join(file.file_name())).unwrap();
        } else if std::fs::copy(file.path(), &to.join(file.file_name())).is_err() {
            println!("Failed to copy files to game directory.");
            std::process::exit(1);
        }
    }
    Ok(())
}