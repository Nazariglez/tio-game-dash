#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate toml;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub server: ConfigServer,
    pub log: ConfigLog,
    pub database: ConfigDatabase,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigServer {
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigDatabase {
    pub user: String,
    pub password: String,
    pub host: String,
    pub name: String
}

pub type ConfigLog = HashMap<String, String>;

impl Config {
    fn default() -> Config {
        Config{
            server: ConfigServer {
                ip: "127.0.0.1".to_string(),
                port: 8000,    
            },
            
            log: (|| {
                [
                    ("main", "info"),
                    ("tarentola", "trace"),
                    ("actix_web", "debug")
                ].into_iter()
                    .map(|(k,v)| {
                        (k.to_string(),v.to_string())
                    })
                    .collect::<HashMap<_,_>>()
            })(),

            database: ConfigDatabase {
                user: "".to_string(),
                password: "".to_string(),
                host: "localhost".to_string(),
                name: "".to_string(),
            }
        }
    }
}

lazy_static! {
    static ref INITIATED:Mutex<bool> = Mutex::new(false);
    static ref GLOBAL_CONFIG:Mutex<Config> = Mutex::new(Config::default());
}

pub fn init(path: String) {
    if !can_init() {
        panic!("Config already INITIATED");
    }

    let mut config_file = File::open(path.clone())
        .unwrap_or_else(|_err| create_default_config(path.clone()));
    
    let mut content = String::new();
    config_file.read_to_string(&mut content)
        .expect("Something went wrong reading the config file");

    let c:Config = match toml::from_str(&content) {
        Ok(c) => c,
        Err(err) => panic!("Error parsing config file: {}", err)
    };

    *GLOBAL_CONFIG.lock().unwrap() = c;
    *INITIATED.lock().unwrap() = true;
}

pub fn create_default_config(path:String) -> File {
    println!("Creating a new config file: {}", path);
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(path)
        .unwrap();

    let toml_config = toml::to_string(&Config::default()).unwrap();

    f.write_all(toml_config.as_bytes()).unwrap();
    f.sync_data().unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    f
}

pub fn get() -> Config {
    (*GLOBAL_CONFIG.lock().unwrap()).clone()
}

fn can_init() -> bool {
    !*(INITIATED.lock().unwrap())
}

