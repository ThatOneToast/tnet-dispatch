use std::{fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub first_time_use: bool,
}

impl Config {
    pub fn save(&self) -> Result<(), toml::ser::Error>{
        let tnet_dispatch: PathBuf = PathBuf::from(env!("HOME")).join(".tnet").join("dispatch");
        std::fs::create_dir_all(&tnet_dispatch).expect("Failed to create all directories.");
        let config_path = tnet_dispatch.join("config.toml");
        let toml_string = toml::to_string(self)?;
        std::fs::write(&config_path, toml_string).expect("Failed to write to file.");
        Ok(())
    }
    
    pub fn get_data_path(&self) -> PathBuf {
        let tnet_dispatch = PathBuf::from(env!("HOME")).join(".tnet").join("dispatch");
        let data = tnet_dispatch.join("DATA");
        std::fs::create_dir_all(&data).expect("Failed to create all directories.");
        data
    }
    
    pub fn get_available_projects(&self) -> Vec<String> {
        let data_path = self.get_data_path();
        
        // Get all entries in the data directory
        let entries = match std::fs::read_dir(&data_path) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Failed to read data directory: {}", e);
                return Vec::new();
            }
        };
        
        // Filter for directories and convert to project names
        let mut projects = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            projects.push(name.to_string());
                        }
                    }
                }
            }
        }
        
        projects
    }
}

impl Default for Config {
    fn default() -> Self {
        let tnet_dispatch: PathBuf = PathBuf::from(env!("HOME")).join(".tnet").join("dispatch");
        std::fs::create_dir_all(&tnet_dispatch).unwrap_or_else(|e| {
            eprintln!("Failed to create directory structure: {}", e);
        });
        let config_path = tnet_dispatch.join("config.toml");
        if !config_path.exists() {
            let default_config = Config {
                first_time_use: true,
            };
            let toml_string =
                toml::to_string(&default_config).expect("Failed to serialize default config");
            std::fs::write(&config_path, toml_string).expect("Failed to write default config file");
            return default_config;
        }
        let mut contents = File::open(&config_path).expect("Failed to open config file");
        let mut buffer = String::new();
        contents
            .read_to_string(&mut buffer)
            .expect("Failed to read config file");
        toml::from_str(buffer.as_str()).expect("Failed to parse config file")
    }
}
