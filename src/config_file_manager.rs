use std::{env, fs, path::Path};
use crate::config::Config;

pub fn get_config_dir() -> String {
    let home_directory = match env::home_dir() {
        Some(home_directory) =>  home_directory,
        None => {
            println!("Could not get home dir!");
            return "".to_string();
        }
    };
    let path_to_config = home_directory.join(".config").join("batsignal-rs");
    // create path if it does not exit ~/.config/batsignal-rs
    match fs::create_dir_all(&path_to_config) {
        Ok(()) => (),
        Err(error) => {
            eprintln!("Could not create the path to the config file");
            eprintln!("{:?}", path_to_config); 
            eprintln!("{:?}", error);
            return "".to_string();
        }
    };
    path_to_config.join("config.toml").display().to_string()
}


pub fn parse_toml(path: &str) -> Config {
    fs::read_to_string(path)
        .ok()
        .and_then(|file| { toml::from_str(&file).ok()})
        .unwrap_or_else(|| { 
            eprintln!("Failed to load config.toml; Check Syntax\nusing defaults");
            Config::default()})
}


pub fn print_config_toml_with_message(config: &Config, message: &str) -> Result<(), toml::ser::Error> {
    println!("{}\n",message);
    let toml = toml::to_string_pretty(config)?;
    println!("{toml}");
    Ok(())
}

pub fn handle_config_file(file_path: &str) -> Config {
    let path = Path::new(file_path);
    if !path.exists() {
        println!("Could not find config.toml, Using Defaults");
        return Config::default();
    }
    parse_toml(file_path)
}

pub fn create_default_config_file(file_path: &str){
    let path = Path::new(file_path);
    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path) {
            Ok(_file) => {
                match toml::to_string_pretty(&Config::default()) {
                    Ok(toml_) => {
                        match fs::write(file_path, toml_) {
                            Ok(()) => (),
                            Err(error) => eprintln!("{}", error)
                        }
                    },
                    Err(error) => eprintln!("{}", error)
                };
            },
            Err(error) => eprintln!("{}", error)
        };
    println!("Creating a config file using defaults");

}