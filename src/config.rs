
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
        for argument in env::args_os() {
            println!("{argument:?}");
        }

        let s = Server {
            host: "0.0.0.0".to_string(),
            port: 8080,
        };

        return Self {server: s};
    }
}