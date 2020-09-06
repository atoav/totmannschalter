use std::fs::File;
use std::path::PathBuf;
use std::result::Result;
use std::default::Default;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use dialoguer::Confirm;
use dirs;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::Mailbox;

use crate::*;

/// Config Struct, gets serialized/deserialized from toml and consists of multiple
/// fields and sections
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub mail: Mail,
    pub endpoint: Vec<Endpoint>,
}

impl Default for Config {
    fn default() -> Self { 
        Self {
            mail: Mail::default(),
            endpoint: vec![Endpoint::default()]
        }
    }
}

/// Holds the configuration for the SMTP connection used
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mail {
    pub adress: String,
    pub port: usize,
    pub user: String,
    pub password: String,
    pub sender_mail: Mailbox
}

impl Default for Mail {
    fn default() -> Self { 
        Self {
            adress: "smtp.example.com".to_string(),
            port: 587,
            user: "user@example.com".to_string(),
            password: "1234".to_string(),
            sender_mail: "Usu Usero <user@example.com>".parse().unwrap()
        }
    }
}

impl Mail {
    pub fn send(&self, email: Message) -> Result<(), MailError>{
        let creds = Credentials::new(self.user.clone(), self.password.clone());
        let mailer = SmtpTransport::relay(&self.adress)
                                    .unwrap()
                                    .credentials(creds)
                                    .build();
        mailer.send(&email)?;
        Ok(())
    }
}

/// Holds the configuration for a Endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct Endpoint {
    pub name: Option<String>,
    pub url: String,
    pub interval: Option<usize>,
    pub contact_mail: Mailbox
}

impl Default for Endpoint {
    fn default() -> Self { 
        Self {
            name: Some("Unnamed URL".to_string()),
            url: "https://request.this.url.com".to_string(),
            interval: Some(60*10),
            contact_mail: "Ms. Ada Mini Strator <admin@this.url.com>".parse().unwrap(),
        }
    }
}

impl Config {

    pub fn initialize() -> Result<Config, ConfigError> {
        let config_path = get_config_path();
        let mut config_folder = get_config_path();
        config_folder.pop();

        if !config_exists() {
            // Config doesn't exist, ask if a new one should be made
            println!("{}\n No config file found at {}", style(" Note ").black().on_bright().on_yellow(), style(&config_path.to_string_lossy()).underlined());
            let q = format!("{}", style(" Should a default configuration be created at that path?").green());
            if Confirm::new().with_prompt(q).interact()? {
                std::fs::create_dir_all(&config_folder)?;
                println!(" {}{}", style("[✓] Created config directory at ").green(), &config_folder.to_string_lossy());
                let config = Config::default();
                let toml = toml::to_string(&config).unwrap();
                let mut file = File::create(&config_path)?;
                file.write_all(toml.as_bytes())?;
                println!(" {}{}", style("[✓] Written default config to ").green(), &config_path.to_string_lossy());
                std::process::exit(0);

            } else {
                std::process::exit(0);
            }
        }else{
            // Config exists, read it
            let mut file = File::open(&config_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            match toml::from_str(&contents) {
                Ok(config) => Ok(config),
                Err(e) => Err(ConfigError::ReadError(config_path.to_string_lossy().to_string(), e.to_string()))
            }
        }
    }

}


pub fn get_config_path() -> PathBuf {
    match dirs::config_dir() {
        Some(mut p) => {
            if cfg!(target_os = "linux") && p.starts_with("/root/.config"){ 
                p = PathBuf::from(p.to_string_lossy().replace("/root/.config", "/etc"));
            }
            p.push(env!("CARGO_PKG_NAME"));
            p.push("config.toml");
            p
        },
        None => {
            eprintln!("{} Couldn't get a default config directory from the OS", style(" Error ").on_red());
            std::process::exit(1);
        }
    }
}

pub fn config_exists() -> bool {
    get_config_path().is_file()
}

pub fn config_directory_exists() -> bool {
    let mut config_path = get_config_path();
    config_path.pop();
    config_path.is_dir()
}