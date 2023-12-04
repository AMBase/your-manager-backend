
use std::env;





#[derive(Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct Config {
    pub server: Server,
}

impl Config {
    pub fn new() -> Self {
        let s = Server {
            host: env::var("HOST").unwrap_or("0.0.0.0".to_string()),
            port: env::var("PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap(),
        };

        return Self {server: s};
    }
}