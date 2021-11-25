use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use crate::util::path;


static mut CONFIG: Option<Config> = None;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub file_upload_path: String,
}

pub fn init() {
    let p = path::this();
    let path = format!("{}/conf/blogs.toml", p);
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("打开文件异常, 异常信息为: {}", e)
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e)
    };
    let config: Config = toml::from_str(&str_val).unwrap();
    unsafe { CONFIG = Some(config) }
}

pub fn this() -> &'static Config {
    unsafe {
        match &CONFIG {
            None => panic!("配置文件加载异常!"),
            Some(v) => v
        }
    }
}