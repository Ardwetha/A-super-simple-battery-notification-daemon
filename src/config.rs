use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ConfigData {
    pub battery_name: String,
    pub alerts: Vec<Alerts>,
}



#[derive(Clone, Deserialize)]
pub struct Alerts {
    pub capacity_level: u16,
    pub warning_level: String,
    pub warning_text: String,
}

impl ConfigData {
    pub fn load() -> Result<Self, config::ConfigError> {
       let config_dir_path_opt = dirs::config_dir();
        if config_dir_path_opt.is_none() {
            return Err(config::ConfigError::NotFound("Config not found".to_string()));
        }
        let mut config_dir_path = config_dir_path_opt.unwrap();
        
        config_dir_path.push("Ass/config.toml");
       let config_builder = config::Config::builder().
            add_source(config::File::with_name(&config_dir_path.into_os_string().into_string().unwrap())).
            build()?;
        config_builder.try_deserialize()
    }
}

